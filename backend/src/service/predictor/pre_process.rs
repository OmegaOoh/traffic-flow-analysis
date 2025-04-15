

use chrono::{DateTime, FixedOffset, Timelike};


pub fn is_weather_valid(weather: &str) -> bool {
    matches!(weather, "Clear" | "Cloudy" | "Rain" | "Low Visibility")
}

pub(crate) fn pre_process(time: DateTime<FixedOffset>, weather: String) -> Result<(f64, f64), String> {
    let t = time.hour() as f64 + (time.minute() as f64)/60.0;
    let w = match weather.as_str() {
        "Clear" => 0.0,
        "Cloudy" => 1.0,
        "Rain" => 2.0,
        "Low Visibility" => 3.0,
        _ => return Err("Weather is not Supported".to_string()),
    };
    Ok((t as f64, w as f64))
}