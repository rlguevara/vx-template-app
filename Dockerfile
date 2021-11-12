FROM node:17 AS schema
WORKDIR /usr/src/app
COPY . .
RUN npm install -g graphqurl
SHELL ["/bin/bash", "-c"] 
RUN set -o allexport; source .env; set +o allexport;./update_schema.sh

FROM rust:1.56 AS builder
WORKDIR /usr/src/myapp
COPY --from=schema usr/src/app .
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN trunk build --release

FROM nginx:1.21.3
COPY --from=builder /usr/src/myapp/dist /usr/share/nginx/html
