use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::openapi;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::types::BigDecimal;

use crate::service::database_connection::Logs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
pub struct VehicleDesc {
    time: String,
    count_m: f64,
    count_c: f64,
    count_h: f64
}


#[openapi(tag="Descriptive")]
#[get("/desc/vehicle?<day_of_week>")]
pub async fn get_all_vehicle(
    db: &Logs, day_of_week: Option<u8>) -> Result<Json<Vec<VehicleDesc>>, Status>
{   
    let mut sql = String::from (
        "SELECT
            CASE
                WHEN MINUTE(time) < 15 THEN
                    TIME_FORMAT(time, '%H:00')
                WHEN MINUTE(time) >= 15 AND MINUTE(time) < 45 THEN
                    CONCAT(LPAD(HOUR(time), 2, '0'), ':30')
                ELSE
                    TIME_FORMAT(DATE_ADD(time, INTERVAL 1 HOUR), '%H:00')
            END AS time_30min_interval,
            AVG(motorcycles) AS motorCounts,
            AVG(cars) AS carCounts,
            AVG(heavyVehicles) AS heavyVehicleCounts
          FROM
            vehicleCounts
         "
    );
    
    if day_of_week.is_some() {
        let d = day_of_week.unwrap();
        if d < 1 || d > 7 {
            return Err(Status::BadRequest);
        }
        
        sql.push_str("WHERE DAYOFWEEK(time) = ");
        sql.push_str(&d.to_string());
    }

    sql.push_str("        
    GROUP BY
        time_30min_interval
    ORDER BY
        time_30min_interval;"
    );
    
    let rows = sqlx::query(&sql)
    .fetch_all(&db.0)
    .await
    .unwrap_or_default();
    
    // Map the results to struct
    let vehicle_counts: Vec<VehicleDesc> = rows.into_iter()
        .map(|row| VehicleDesc {
            time: row.try_get("time_30min_interval").unwrap_or_default(),
            count_m: row.try_get::<BigDecimal, _>("motorCounts").unwrap_or_default().to_string().parse::<f64>().unwrap_or_default(),
            count_c: row.try_get::<BigDecimal, _>("carCounts").unwrap_or_default().to_string().parse::<f64>().unwrap_or_default(),
            count_h: row.try_get::<BigDecimal, _>("heavyVehicleCounts").unwrap_or_default().to_string().parse::<f64>().unwrap_or_default()
        })
        .collect();

    Ok(Json(vehicle_counts))
}