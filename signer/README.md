# Signer

A separate service which upon request, creates and signs a transaction which is sent to the relayer service. The signer service is run on an isolated VPC to prevent unauthorized requests and uses a key management service to keep the signing key secure.

## Installation

```bash
$ yarn install
```

## Running the app

```bash
# development
$ yarn run start

# production mode
$ yarn run start:prod
```

## Test

```bash
# unit tests
$ yarn run test

# e2e tests
$ yarn run test:e2e

# test coverage
$ yarn run test:cov
```
