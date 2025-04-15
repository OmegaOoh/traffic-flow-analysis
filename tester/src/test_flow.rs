#[cfg(test)]
mod tests {
    use tokio;
    use reqwest;
    use serde_json::{json, Value};
    use dotenv::dotenv;
    use std::env::var as env_var;

    #[tokio::test]
    async fn test_flow_endpoint_all_valid() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/flow", endpoint);
        let data = json!({
            "time": "2023-05-15T14:30:00+07:00",
            "weather_cond": "Clear"
        });

        let response = client.post(url).body(serde_json::to_string(&data).unwrap()).header("Content-Type", "application/json").send().await?;

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.get("flow").is_some());
        assert!(body_json.get("flow").unwrap().is_f64());
        Ok(())
    }
}