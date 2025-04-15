use std::sync::LazyLock;
use std::env;
use std::path::PathBuf;
use tch::CModule;

pub fn init_model() {
    // Initialize model by call them once
    let _ = TOTAL_VEHICLE_NUM_MODEL;
    let _ = MOTORCYCLE_NUM_MODEL;
    let _ = CAR_NUM_MODEL;
    let _ = HEAVY_VEHICLE_NUM_MODEL;
    let _ = FLOW_MODEL;
    let _ = DENSITY_MODEL;
}


static MODELS_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    env::current_dir().unwrap().join("src").join("pytorch_models")
});

pub static TOTAL_VEHICLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("total_vehicle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load vehicle count model: {}", e))
});

pub static MOTORCYCLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("motorcycle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load motorcycle count model: {}", e))
});

pub static CAR_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("car_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load car count model: {}", e))
});

pub static HEAVY_VEHICLE_NUM_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("heavy_vehicle_num_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load heavy vehicle count model: {}", e))
});

pub static FLOW_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("traffic_flow_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load traffic flow model: {}", e))
});

pub static DENSITY_MODEL: LazyLock<CModule> = LazyLock::new(|| {
    CModule::load(MODELS_PATH.join("road_density_model.pt"))
        .unwrap_or_else(|e| panic!("Failed to load traffic flow model: {}", e))
});
