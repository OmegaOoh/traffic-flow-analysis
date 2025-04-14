# Traffic Flow Analysis Frontend


## Project Setup
- Install dependencies
>>>>>>> main
```sh
bun install
```

<<<<<<< HEAD
### Compile and Hot-Reload for Development

```sh
bun dev
```
=======
- Create .env file by copy sample.env into `.env` and set `VUE_APP_BASE_URL` into your backend Url
>>>>>>> main

### Type-Check, Compile and Minify for Production

```sh
bun run build
```

<<<<<<< HEAD
=======
### Run Project (DEV)
```sh
bun run dev
```


>>>>>>> main
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

### Lint with [ESLint](https://eslint.org/)

```sh
bun lint
```
