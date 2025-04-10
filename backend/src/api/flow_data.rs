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
struct FlowDesc {
    time: String,
    speed: f64,
}

#[openapi(tag="Descriptive")]
#[get("/desc/flow", rank=2)]
pub fn get_all_flow_docs() -> rocket::serde::json::Json<Vec<FlowDesc>> {
    // Open API documentation
    rocket::serde::json::Json(vec![])
}


#[openapi(tag="Descriptive")]
#[get("/desc/flow")]
pub async fn get_all_flow(
    db: &Logs) -> rocket::serde::json::Json<Vec<FlowDesc>>
{   
    let mut  sql = String::from (
        "SELECT
            CASE
                WHEN MINUTE(time) < 15 THEN
                    TIME_FORMAT(time, '%H:00')
                WHEN MINUTE(time) >= 15 AND MINUTE(time) < 45 THEN
                    CONCAT(LPAD(HOUR(time), 2, '0'), ':30')
                ELSE
                    TIME_FORMAT(DATE_ADD(time, INTERVAL 1 HOUR), '%H:00')
            END AS time_30min_interval,
            AVG(currentSpeed) AS avg_speed
        FROM
            trafficflow
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
    let flow_data: Vec<FlowDesc> = rows.into_iter()
        .map(|row| FlowDesc {
            time: row.try_get("time_30min_interval").unwrap_or_default(),
            speed: row.try_get::<BigDecimal, _>("avg_speed").unwrap_or_default().to_string().parse::<f64>().unwrap_or_default()
        })
        .collect();

    rocket::serde::json::Json(flow_data)
}