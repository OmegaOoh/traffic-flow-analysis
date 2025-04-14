# traffic-flow-analysis
Simple API web application to provide data about traffic flow

## how to run
1. Copy 'sample.env' to '.env'
2. Edit `ALLOWED_ORIGINS` in `.env` file to be your __frontend__ url
3. Edit `url` of `ROCKET_DATABASE` in `.env` file to be your __database__ credentials (using database connection URI).
4. compile and run the server with 
  ```sh
    cargo run
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
  