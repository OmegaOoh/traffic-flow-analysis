# traffic-flow-analysis
Simple API web server to provide data about traffic flow

## how to run
1. Copy `.env.sample` into `.env`
2. Edit `ROCKET_DATABASES__MYSQL_LOGS__URL` in `.env` file to be your __mysql__ database url
3. compile and run the server with 
  ```sh
    cargo run
  ```