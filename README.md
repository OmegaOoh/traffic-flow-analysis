# traffic-flow-analysis
Simple API web application to provide data about traffic flow

## how to run
1. Go into the backend directory
1. Copy 'sample.env' to '.env'
2. Edit `ALLOWED_ORIGINS` in `.env` file to be your __frontend__ url
3. Edit `url` of `ROCKET_DATABASE` in `.env` file to be your __database__ credentials (using database connection URI).
4. Edit `url` in `rocket.toml` file to be your **mysql** database url
5. Install and Add PATH to [LibTorch](https://pytorch.org/get-started/locally/) (Setup guide can be found [here](https://rustrepo.com/repo/LaurentMazare-tch-rs-rust-machine-learning)) **or** follow [this if you have Pytorch installed](https://necrashter.github.io/tch-rs-install-from-pytorch)
6. compile and run the server with 

  ```sh
    cargo run
  ```
7. Go to frontend directory
8. Copy 'sample.env' to '.env'
9. Change `VITE_APP_ENDPOINT` to be your __backend__ including path to API Endpoint. Default Path is `/api/v2`
9. Install and Run __frontend__ with 

  ```sh
    npm install
    npm start
  ```
  
## MYSQL database required Table and Columns
1. weatherData
    1. time: DateTime
    2. weatherDesc: VarChar(255)
2. trafficflow
    1. time: DateTime
    2. currentSpeed: INT
3. vehicleCounts
    1. time: DateTime
    2. motorcyclesCount: INT
    3. CarCount: INT
    4. heavyVehicleCount: INT
    
## Prediction Model
Prediction Model may not be accurate due to limited data and complexity of traffic flow. However, it can provide a general idea of traffic flow patterns and help in making informed decisions.
Model can be trained using Pytorch and replace in `/backend/src/pytorch_models/`

Note: Jupyter Notebook used to trained the model use on git will be provided in the repository.