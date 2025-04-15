use std::sync::LazyLock;
use chrono::{DateTime, Duration, FixedOffset};
use tch::{CModule, Tensor, Device};

use crate::service::predictor::pre_process::pre_process;
// Trait and Struct
pub trait ModelInterface {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String>;
}

pub struct VehicleCountInferencer;
pub struct MotorcycleCountInferencer;
pub struct CarCountInferencer;
pub struct HeavyVehicleCountInferencer;
pub struct FlowInferencer;

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
