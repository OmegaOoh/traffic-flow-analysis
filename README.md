# Traffic Flow Analysis
[![Rust Unit Test Backend](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/rust_unit_test_backend.yml/badge.svg)](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/rust_unit_test_backend.yml)
[![Frontend Unit Test](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/node_frontend_unit_test.yml/badge.svg)](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/node_frontend_unit_test.yml)

A simple API web application providing data about traffic flow is built with a Rust backend (Rocket) and a Vue.js/Vite frontend.

## Prerequisites

Ensure you have the following installed:

* **Rust and Cargo:** For the backend. Follow the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* **Node.js and a JavaScript Package Manager:** For the frontend (npm, Yarn, or Bun). [https://nodejs.org/](https://nodejs.org/)
* **MySQL Database:** The backend requires a MySQL instance.
* **LibTorch:** For the prediction model used by the backend. Installation guide: [https://pytorch.org/get-started/locally/](https://pytorch.org/get-started/locally/). 
Setup guides for `tch-rs`: [https://rustrepo.com/repo/LaurentMazare-tch-rs-rust-machine-learning](https://rustrepo.com/repo/LaurentMazare-tch-rs-rust-machine-learning) or [https://necrashter.github.io/tch-rs-install-from-pytorch](https://necrashter.github.io/tch-rs-install-from-pytorch).

## Database Schema

The project requires a MySQL database with the following tables and columns:

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

## Project Setup

### Backend Setup

1.  **Navigate to the backend directory:**
    ```sh
    cd backend
    ```
2.  **Configure Environment:**
    Copy the sample environment file:
    ```sh
    cp sample.env .env
    ```
3.  **Edit `.env`:**
    Open the newly created `.env` file and set the following:
    * `ALLOWED_ORIGINS`: Your **frontend** URL(s) for CORS.
    * `ROCKET_DATABASE`: Your **database** credentials using a database connection URI (e.g., `mysql://user:password@host:port/database`).
5.  **LibTorch Configuration:**
    Ensure LibTorch is installed and configured correctly for the Rust `tch-rs` library to find it (refer to LibTorch prerequisites section above).

### Frontend Setup

1.  **Navigate back to the root directory, then into the frontend directory:**
    ```sh
    cd ..
    cd frontend
    ```
    (Alternatively, navigate directly from the root: `cd frontend`)
2.  **Configure Environment:**
    Copy the sample environment file:
    ```sh
    cp sample.env .env
    ```
3.  **Edit `.env`:**
    Open the newly created `.env` file and change `VITE_APP_ENDPOINT` to be your **backend** URL, including the path to the API endpoint. The default path is `/api/v2`. Example: `VITE_APP_ENDPOINT=http://localhost:8000/api/v2`.
4.  **Install Dependencies:**
    Install the frontend dependencies using your preferred package manager:
    ```sh
    npm install
    ```
    (Or `yarn install` / `bun install`)

## Running the Application (Development)

To run the full application, you need to start both the backend and the frontend simultaneously, typically in separate terminal windows.

1.  **Start the Backend:**
    Open a terminal window, navigate to the `backend` directory, and run:
    ```sh
    cd backend
    cargo run
    ```
2.  **Start the Frontend:**
    Open a **second** terminal window, navigate to the `frontend` directory, and run:
    ```sh
    cd frontend
    npm run dev
    ```
    (Or `yarn run dev` / `bun run dev`)

The backend API should now be running (defaulting to `http://localhost:8000`), and the frontend development server should be running (defaulting to `http://localhost:5173` for Vite development build).

## Prediction Model

The backend includes a prediction model based on PyTorch/LibTorch.

The prediction model is used to forecast traffic flow based on historical data and weather conditions.

* The model provided in project repository is trained based on our collected data, and **does not update automatically**, Please train new model to have the prediction based on your data.
* The model may have varying accuracy depending on the training data size and traffic conditions.
* The model is loaded from files within the `/backend/src/pytorch_models/` directory.
* The model can be trained using PyTorch. You can train a new model and replace the files in the specified directory.

*Note: A Jupyter Notebook used for training the model is provided in the root directory of the project in `notebook.zip`.*