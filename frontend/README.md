# Traffic Flow Analysis Frontend


## Project Setup
- Install dependencies
```sh
bun install
```

- Create .env file by copy sample.env into `.env` and set `VUE_APP_BASE_URL` into your backend Url

### Type-Check, Compile and Minify for Production

```sh
bun run build
```

### Run Project (DEV)
```sh
bun run dev
```


### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
bun test:unit
```

### Run End-to-End Tests with [Cypress](https://www.cypress.io/)

```sh
bun test:e2e:dev
```

This runs the end-to-end tests against the Vite development server.
It is much faster than the production build.

But it's still recommended to test the production build with `test:e2e` before deploying (e.g. in CI environments):

```sh
bun run build
bun test:e2e
```