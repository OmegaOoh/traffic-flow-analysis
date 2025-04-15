#[cfg(test)]
mod tests {
    use tokio;
    use reqwest;
    use serde_json::{json, Value};
    use dotenv::dotenv;
    use std::env::var as env_var;

    #[tokio::test]
    async fn test_count_endpoint_all_valid() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.get("count").is_some());
        assert!(body_json.get("vehicle_type").is_some());
        assert_eq!(body_json.get("vehicle_type").unwrap().as_str(), Some("Motorcycle, Car, HeavyVehicle"));

        Ok(())
    }

    #[tokio::test]
    async fn test_count_endpoint_invalid_time() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count", endpoint);
        let data = json!({
            "time": "a-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;

        assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

        Ok(())
    }

    #[tokio::test]
    async fn test_count_endpoint_invalid_weather() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Candy"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

        Ok(())
    }

    #[tokio::test]
    async fn test_count_endpoint_get() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.get(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;

        assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);

        Ok(())
    }

    #[tokio::test]
    async fn test_count_with_type_valid() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count/car", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.get("count").is_some());
        assert!(body_json.get("vehicle_type").is_some());
        assert_eq!(body_json.get("vehicle_type").unwrap().as_str(), Some("Car"));
        
        Ok(())
    }
    #[tokio::test]
    async fn test_count_with_type_invalid() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/count/helicopter", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;
        assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
        
        Ok(())
    }
}