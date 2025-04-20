# Traffic Flow Analysis Tester

## Prerequisite
* **Rust and Cargo:** For the backend. Follow the official installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
* **Node.js and Bun Javascript Package Manager:** For the frontend. [https://nodejs.org/](https://nodejs.org/), [https://bun.sh](https://bun.sh).
* **MySQL Database:** The backend requires a MySQL instance.
* **LibTorch:** For the prediction model used by the backend. Installation guide: [https://pytorch.org/get-started/locally/](https://pytorch.org/get-started/locally/). 
Setup guides for `tch-rs`: [https://rustrepo.com/repo/LaurentMazare-tch-rs-rust-machine-learning](https://rustrepo.com/repo/LaurentMazare-tch-rs-rust-machine-learning) or [https://necrashter.github.io/tch-rs-install-from-pytorch](https://necrashter.github.io/tch-rs-install-from-pytorch).


## Setup Tester environment (.env)
1. Copy `sample.env` to `.env`.
2. Edit ENDPOINT in `.env` to match your configuration, ENDPOINT should be a valid URL to the exact path of the traffic flow analysis API ENDPOINT. 

## Features

- Run Backend Unit Tests
- Test API Endpoints
- Run Frontend Unit Tests
- Run Frontend UI Tests

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
