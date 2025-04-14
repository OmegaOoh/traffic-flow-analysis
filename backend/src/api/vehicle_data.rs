use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::openapi;
use rocket_okapi::okapi::schemars;
use schemars::JsonSchema;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::Row;
use sqlx::types::BigDecimal;

use crate::service::database_connection::Logs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(crate = "rocket::serde")]
struct VehicleDesc {
    time: String,
    count_m: f64,
    count_c: f64,
    count_h: f64
}

#[openapi(tag="Descriptive")]
#[get("/desc/vehicle", rank=2)]
pub fn get_all_vehicle_docs() -> rocket::serde::json::Json<Vec<VehicleDesc>> {
    // Open API documentation
    rocket::serde::json::Json(vec![])
}


#[openapi(tag="Descriptive")]
#[get("/desc/vehicle")]
pub async fn get_all_vehicle(
    db: &Logs) -> rocket::serde::json::Json<Vec<VehicleDesc>>
{   
    let sql = String::from (
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
        GROUP BY
            time_30min_interval
        ORDER BY
            time_30min_interval;
         "
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

    rocket::serde::json::Json(vehicle_counts)
}