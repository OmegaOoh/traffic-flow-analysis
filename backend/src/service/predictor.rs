use std::sync::LazyLock;
use chrono::{DateTime, Duration, FixedOffset, Timelike};
use tch::{CModule, Tensor, Device, IValue};
use std::env;
use std::path::PathBuf;

static MODELS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    env::current_dir().unwrap().join("src").join("pytorch_models")
});

pub static VEHICLE_COUNT_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("vehicle_count_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load vehicle count model: {}", e))
});

fn pre_process(time: DateTime<FixedOffset>, weather: String) -> (f64, f64) {
    let t = time.hour() as f64 + (time.minute() as f64)/60.0;
    let w = match weather.as_str() {
        "Clear" => 0.0,
        "Cloudy" => 1.0,
        "Rainy" => 2.0,
        "Foggy" => 3.0,
        _ => 0.0,
    };
    (t as f64, w as f64)
}

pub fn is_weather_valid(weather: &str) -> bool {
    matches!(weather, "Clear" | "Cloudy" | "Rainy" | "Foggy")
}

pub fn count_interference(time: DateTime<FixedOffset>, weather: String, is_weekend: bool) -> f64 {
    let (hour, weather_code) = pre_process(time, weather.clone());
    let day = if is_weekend { 2.0 } else { 6.0 }; // TODO, remove this conversion 

    let past_time = time - Duration::minutes(30);
    let (past_hour, _) = pre_process(past_time, weather);

    let input_data: Vec<f32> = vec![
        past_hour as f32, weather_code as f32, day as f32,
        hour as f32, weather_code as f32, day as f32,
    ];

    let tensor = Tensor::from_slice(&input_data)
        .reshape(&[1, 2, 3])
        .to_device(Device::cuda_if_available());

    println!("Input Tensor shape: {:?}", tensor.size());
    println!("Input Tensor data: {:?}", tensor.data());

    let output = VEHICLE_COUNT_MODEL.forward_ts(&[tensor]).unwrap();
    output.double_value(&[0])
}
