# Traffic Flow Analysis Tester

## Setup Tester environment (.env)
1. Copy `sample.env` to `.env`.
2. Edit ENDPOINT in `.env` to match your configuration, ENDPOINT should be a valid URL to the exact path of the traffic flow analysis API ENDPOINT. 
3. Edit JS_PACKAGE_MANAGER in `.env` to match your configuration, only `npm`,`pnpm`, `yarn`, and `bun` are supported.

## Features

- Run Backend Unit Tests
- Test API Endpoints
- Run Frontend Unit Tests

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
