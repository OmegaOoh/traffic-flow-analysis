# Traffic Flow Analysis Tester

## Setup Tester environment (.env)
1. Copy `sample.env` to `.env`.
2. Edit ENDPOINT in `.env` to match your configuration, ENDPOINT should be a valid URL to the exact path of the traffic flow analysis API ENDPOINT. 

## Features

- Run Backend Unit Tests
- Test API Endpoints
- Run Frontend Unit Tests (If bun is not installed this will not be run successfully and will result in an error)

## Running the Tester

* To run the full tester, execute the following command:

  ```sh
  cargo run
  ```

* To run API Endpoints, execute the following command:

  **This required the backend server to be running**

  ```sh
  cargo test
  ```
