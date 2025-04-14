use chrono::{DateTime, FixedOffset};

use crate::service::prediction_model;
use crate::service::predictor::{
    VehicleCountInferencer,
    MotorcycleCountInferencer,
    CarCountInferencer,
    HeavyVehicleCountInferencer,
    FlowInferencer,
    ModelInterface,
    model_inference
};

impl ModelInterface for VehicleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&prediction_model::TOTAL_VEHICLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for MotorcycleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&prediction_model::MOTORCYCLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for CarCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&prediction_model::CAR_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for HeavyVehicleCountInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&prediction_model::HEAVY_VEHICLE_NUM_MODEL, time, weather)
    }
}


impl ModelInterface for FlowInferencer {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String> {
        model_inference(&prediction_model::FLOW_MODEL, time, weather)
    }
}
