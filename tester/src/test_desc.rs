#[cfg(test)]
mod test {
    use tokio;
    use reqwest::{self, StatusCode};
    use serde_json::Value;
    use dotenv::dotenv;
    use std::env::var as env_var;
    
    #[tokio::test]
    async fn test_get_weather_data() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/desc/weather", endpoint);
        
        let response = client.get(url).send().await?;
        
        assert_eq!(response.status(), StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.is_array());
        assert!(body_json.as_array().unwrap().len() > 0);
        assert!(body_json.as_array().unwrap()[0].is_object());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("weather"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("weather").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("weather").unwrap().is_string());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("count"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count").unwrap().is_number());
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_get_flow_data() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/desc/flow", endpoint);
        
        let response = client.get(url).send().await?;
        
        assert_eq!(response.status(), StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.is_array());
        assert!(body_json.as_array().unwrap().len() > 0);
        assert!(body_json.as_array().unwrap()[0].is_object());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("time"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("time").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("time").unwrap().is_string());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("speed"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("speed").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("speed").unwrap().is_number());    
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_get_vehicle_data() -> Result<(), reqwest::Error> {
        dotenv().ok();
        let endpoint: String = env_var("ENDPOINT").expect("ENDPOINT not found");
        let client = reqwest::Client::new();
        let url = format!("{}/desc/vehicle", endpoint);
        
        let response = client.get(url).send().await?;
        
        assert_eq!(response.status(), StatusCode::OK);
        let body_json: Value = response.json().await?;
        assert!(body_json.is_array());
        assert!(body_json.as_array().unwrap().len() > 0);
        assert!(body_json.as_array().unwrap()[0].is_object());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("time"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("time").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("time").unwrap().is_string());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("count_m"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_m").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_m").unwrap().is_number());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("count_c"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_c").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_c").unwrap().is_number());
        
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().contains_key("count_h"));
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_h").is_some());
        assert!(body_json.as_array().unwrap()[0].as_object().unwrap().get("count_h").unwrap().is_number());
        
        Ok(())
    }
}
