use std::sync::LazyLock;

use chrono::{DateTime, Duration, FixedOffset, Timelike};
use tch::{CModule, Tensor, Device};
use crate::service::prediction_model;


pub fn init_model() {
    // Initialize model by call them once
    let _ = prediction_model::TOTAL_VEHICLE_NUM_MODEL;
    let _ = prediction_model::MOTORCYCLE_NUM_MODEL;
    let _ = prediction_model::CAR_NUM_MODEL;
    let _ = prediction_model::HEAVY_VEHICLE_NUM_MODEL;
    let _ = prediction_model::FLOW_MODEL;
    let _ = prediction_model::DENSITY_MODEL;
}

pub fn is_weather_valid(weather: &str) -> bool {
    matches!(weather, "Clear" | "Cloudy" | "Rain" | "Low Visibility")
}

fn pre_process(time: DateTime<FixedOffset>, weather: String) -> Result<(f64, f64), String> {
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


pub(crate) fn model_inference(model: &LazyLock<CModule>,time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
    let (hour, weather_code) = match pre_process(time, weather.clone()) {
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let past_time = time - Duration::minutes(30);
    let (past_hour, _) = match pre_process(past_time, weather){
        Ok(r) => r,
        Err(e) => return Err(e)
    };

    let input_data: Vec<f32> = vec![
        past_hour as f32, weather_code as f32,
        hour as f32, weather_code as f32
    ];

    let tensor = Tensor::from_slice(&input_data)
        .reshape(&[1, 2, 2])
        .to_device(Device::cuda_if_available());

    let output = model.forward_ts(&[tensor]).unwrap();
    Ok(output.double_value(&[0]))
}


// Trait and Struct
pub trait ModelInterface {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String>;
}

pub struct VehicleCountInferencer;
pub struct MotorcycleCountInferencer;
pub struct CarCountInferencer;
pub struct HeavyVehicleCountInferencer;
pub struct FlowInferencer;
pub struct DensityInferencer;