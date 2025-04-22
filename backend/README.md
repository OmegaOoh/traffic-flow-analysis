# Traffic Flow Analysis Backend

[![Rust Unit Test Backend](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/rust_unit_test_backend.yml/badge.svg)](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/rust_unit_test_backend.yml)

## Requirements

* Rust Toolchain ([rustup](https://rustup.rs/) is recommended)
* A MySQL Database instance
* LibTorch (for the prediction model)

## Setup and Running

1.  **Environment Configuration (.env):**
    Copy the sample environment file to create your local configuration file:
    ```sh
    cp sample.env .env
    ```
    Open the `.env` file and configure the following variables:
    * `ALLOWED_ORIGINS`: Set this to your **frontend** URL(s) for Cross-Origin Resource Sharing (CORS).
    * `ROCKET_DATABASE`: Set `url` to your **database** connection URI. This variable is used by Rocket's database pools. Example: `mysql://user:password@host:port/database`. variables `idle_timeout`, `max_connections`, `min_connections` can be changed to your desired values.
    

2.  **Install and Configure LibTorch/Pytorch:**
    The backend uses Rust binding (`tch-rs`) for LibTorch/Pytorch to load and run the prediction model. You need to install LibTorch or Pytorch and ensure that the environment variables are set correctly.

    * `tch-rs` installation guide:
      * Official installation guide (C++ or Java): [https://github.com/LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs)
      * Use `tch-rs` from Pytorch (Python): [https://necrashter.github.io/tch-rs-install-from-pytorch](https://necrashter.github.io/tch-rs-install-from-pytorch)
    * To install LibTorch/Python follow the installation guide: [https://pytorch.org/get-started/locally/](https://pytorch.org/get-started/locally/)

* MacOS have error on dynamic linking, You can fix this by add path to pytorch lib into environment `DYLD_LIBRARY_PATH`.

3.  **Compile and Run the Server:**
    With the dependencies installed and environment configured, compile and run the backend server using Cargo:
    ```sh
    cargo run
    ```
    
    This will start the Rocket web server on `http://localhost:8000` by default, ready to serve API requests.
    
    to build and run the server in release mode:
    ```sh
    cargo build --release
    cargo run --release
    ```
    
    Port of the server can be changed by setting the `ROCKET_PORT` to your desired port environment variable.

## Database Schema

The backend requires a MySQL database with the following tables and columns:

1.  `weatherData`
    * `time`: `DateTime`
    * `weatherDesc`: `VarChar(255)`
2.  `trafficflow`
    * `time`: `DateTime`
    * `currentSpeed`: `INT`
3.  `vehicleCounts`
    * `time`: `DateTime`
    * `motorcycles`: `INT`
    * `cars`: `INT`
    * `heavyVehicles`: `INT`

## Prediction Model

The prediction model is used to forecast traffic flow based on historical data and weather conditions.

* The model provided in project repository is trained based on our collected data, and **does not update automatically**, Please train new model to have the prediction based on your data.
* The model may have varying accuracy depending on the training data size and traffic conditions.
* The model is loaded from files within the `/backend/src/pytorch_models/` directory.
* The model can be trained using PyTorch. You can train a new model and replace the files in the specified directory.

*Note: A Jupyter Notebook used for training the model is provided in the root directory of the project in `notebook.zip`.*