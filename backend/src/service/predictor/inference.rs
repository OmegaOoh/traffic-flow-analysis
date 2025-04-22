use chrono::{DateTime, FixedOffset};

// Trait and Struct
pub trait ModelInterface {
    fn inference(time: DateTime<FixedOffset>, weather: String) -> Result<f64, String>;
}

pub struct VehicleCountInferencer;
pub struct MotorcycleCountInferencer;
pub struct CarCountInferencer;
pub struct HeavyVehicleCountInferencer;
pub struct FlowInferencer;
