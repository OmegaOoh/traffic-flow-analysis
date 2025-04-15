#[cfg(test)]
mod test {
    use chrono::DateTime;
    use crate::service::predictor::pre_process::*;
    
    #[test]
    fn test_is_weather_valid() {
        assert!(is_weather_valid("Clear"));
        assert!(is_weather_valid("Cloudy"));
        assert!(is_weather_valid("Rain"));
        assert!(is_weather_valid("Low Visibility"));
    }
    
    #[test]
    fn test_is_weather_invalid() {
        assert!(!is_weather_valid("Sunny"));
    }
    
    #[test]
    fn test_pre_process_valid_midnight() {
        let datetime_str = "2023-01-01T00:00:00+07:00";
        let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
        let result = pre_process(datetime, String::from("Clear"));
        assert_eq!(result.ok().unwrap(), (0.0, 0.0));
    }
    
    #[test]
    fn test_pre_process_valid_2330() {
        let datetime_str = "2023-01-01T23:30:00+07:00";
        let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
        let result = pre_process(datetime, String::from("Clear"));
        assert_eq!(result.ok().unwrap(), (23.5, 0.0));
    }
    
    #[test]
    fn test_pre_process_valid_all_weather() {
        let datetime_str = "2023-01-01T00:00:00+07:00";
        let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
        assert_eq!(pre_process(datetime, String::from("Clear")).ok().unwrap(), (0.0, 0.0));
        assert_eq!(pre_process(datetime, String::from("Cloudy")).ok().unwrap(), (0.0, 1.0));
        assert_eq!(pre_process(datetime, String::from("Rain")).ok().unwrap(), (0.0, 2.0));
        assert_eq!(pre_process(datetime, String::from("Low Visibility")).ok().unwrap(), (0.0, 3.0));
    }
    
    #[test]
    fn test_pre_process_invalid_weather() {
        let datetime_str = "2023-01-01T23:30:00+07:00";
        let datetime = DateTime::parse_from_rfc3339(datetime_str).unwrap();
        let result = pre_process(datetime, String::from("Candy"));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), String::from("Weather is not Supported"));
    }
}
