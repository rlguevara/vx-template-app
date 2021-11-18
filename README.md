# Vx Template App

## Install [Rust](https://www.rust-lang.org/)

### Using [Rustup](https://rustup.rs/)
Run the following in your terminal, then follow the onscreen instructions:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install  Trunk
```
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

## Update Schema

In order to run update_schema.sh locally you will need to go to the file and uncomment the line below

```
#source '.env'  
```

## Serve locally
```
trunk serve
```

## Setup with Docker

- Install [Docker](https://docs.docker.com/engine/install/ubuntu)

- Install [Docker-Compose](https://docs.docker.com/compose/install)

- Install [Hasura CLI](https://hasura.io/docs/latest/graphql/core/hasura-cli/install-hasura-cli.html)

```
$ curl -L https://github.com/hasura/graphql-engine/raw/stable/cli/get.sh | VERSION=v2.0.8 bash
```

or the one that is being used in the docker-compose file. You can also update by running

```
$ hasura update-cli --version v2.0.8
```

### Usage

- Clone the project

```
$ git clone git@github.com:VertexStudio/vx-template-app
```

- Copy the environment file as **.env**

```
$ cp .env.example .env
```

- Run the development environment with docker-compose

```
$ docker-compose -f docker-compose.dev.yaml up -d
```
- Run the production environment with docker-compose

```
$ docker-compose up -d
```

- Docker settings

After creating the services containers using Docker Compose:
In order to make the Keycloak data persistent in the assigned volume for the Postgres service, you need to stop the containers, set the command in the
Docker-compose file, then start the containers again.

```
'-Dkeycloak.migration.strategy=IGNORE_EXISTING'
```

## Environment configuration

- Default environment file: **.env**
- Example environment file: **.env.example**

| Variable Name               | Description                             |
| --------------------------- | --------------------------------------- |
| HASURA_ENDPOINT             | Hasura API Url                          |
| HASURA_WS_ENDPOINT          | Hasura WS API Url                       |
| DATABASE_HOST               | Postgres database host                  |
| POSTGRES_USER               | Postgres default user                   |
| POSTGRES_PASSWORD           | Postgres default password               |
| POSTGRES_DB                 | Postgres default database name          |
| HOST_ODOO                   | Address of postgres server for Odoo     |
| USER_ODOO                   | Odoo postgres username                  |
| POSTGRES_PASSWORD_ODOO      | Odoo postgres password                  |
| DB_VENDOR                   | Database management vendor              |
| DB_ADDR                     | Address of postgres server for Keycloak |
| DB_DATABASE                 | Keycloak postgres database name         |
| DB_SCHEMA                   | Keycloak postgres schema name           |
| DB_PASSWORD                 | Keycloak postgres database password     |
| KEYCLOAK_USER               | Keycloak default username               |
| KEYCLOAK_PASSWORD           | Keycloak default password               |
| HASURA_GRAPHQL_DATABASE_URL | Postgres Database Url                   |
| HASURA_GRAPHQL_ADMIN_SECRET | Hasura GraphQL Admin Secret             |
| KEYCLOAK_PUBLIC_KEY         | Keycloak RS256 public key               |
| HASURA_GRAPHQL_JWT_SECRET   | JWT secret key                          |