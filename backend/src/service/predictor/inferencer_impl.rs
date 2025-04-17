use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use chrono::{DateTime, Duration, FixedOffset};
use tch::{CModule, Device, Tensor};

use crate::service::predictor::inference::{
    VehicleCountInferencer,
    MotorcycleCountInferencer,
    CarCountInferencer,
    HeavyVehicleCountInferencer,
    FlowInferencer,
    ModelInterface,
};

use super::pre_process::pre_process;


static MODELS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    env::current_dir().unwrap().join("src").join("pytorch_models")
});

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

static TOTAL_VEHICLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("total_vehicle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load vehicle count model: {}", e))
});

static MOTORCYCLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("motorcycle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load motorcycle count model: {}", e))
});

static CAR_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("car_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load car count model: {}", e))
});

static HEAVY_VEHICLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("heavy_vehicle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load heavy vehicle count model: {}", e))
});

static FLOW_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("traffic_flow_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load traffic flow model: {}", e))
});


impl ModelInterface for VehicleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&TOTAL_VEHICLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for MotorcycleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&MOTORCYCLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for CarCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&CAR_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for HeavyVehicleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&HEAVY_VEHICLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for FlowInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&FLOW_MODEL, time, weather)
    }
}
