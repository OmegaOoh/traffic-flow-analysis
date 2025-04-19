# Traffic Flow Analysis Frontend

[![Frontend Unit Test](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/node_frontend_unit_test.yml/badge.svg)](https://github.com/OmegaOoh/traffic-flow-analysis/actions/workflows/node_frontend_unit_test.yml)

This is the frontend repository for the Traffic Flow Analysis project, built with Vue.js.

## Project Setup

**Node.JS is required to run the application.**

1.  **Install Dependencies:**
    Navigate to the project root directory in your terminal and run:
    
    ```sh
    npm install
    ```
    
    ```sh
    yarn install
    ```
    
    ```sh
    bun install
    ```

2.  **Environment Configuration:**
    Create a `.env` file in the project root by copying the provided `sample.env`.
    ```sh
    cp sample.env .env
    ```
    Open the newly created `.env` file and set the `VUE_APP_BASE_URL` variable to the URL of your backend API.

## Development

### Run Development Server

To compile and hot-reload the application for development:

```sh
npm run dev
```

```sh
yarn dev
```

```sh
bun run dev
```

### Build for Production

To build the application for production:

```sh
npm run build
```

```sh
yarn build
```

```sh
bun run build
```

### Run Cypress Tests

To run the Cypress tests:

```sh
npm run test:e2e
```

```sh
yarn test:e2e
```

```sh
bun test:e2e
```

### Run Unit Tests

To run the unit tests:

```sh
npm run test:unit
```

```sh
yarn test:unit
```

```sh
bun test
```
