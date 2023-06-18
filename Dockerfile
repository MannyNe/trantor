# BUILDING THE CLIENT
FROM node:18-alpine as client_builder

RUN corepack enable && corepack prepare pnpm@latest --activate
WORKDIR /opt
RUN mkdir client
WORKDIR /opt/client
COPY ./client ./
RUN pnpm install
RUN pnpm build

# BUILDING THE RUST FILES
FROM rust:1.70.0-alpine as builder

# ADD DEPENDENCIES FOR THE IMAGES
RUN apk add --no-cache musl-dev
RUN apk add pkgconfig
RUN apk add libressl-dev

RUN USER=root cargo new --bin trantor
WORKDIR /trantor
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./config.toml ./config.toml
RUN cargo run -- config.toml
RUN rm src/*.rs
RUN rm ./target/debug/deps/trantor*

ADD . ./

RUN mkdir -p client/build
WORKDIR /trantor/client/build
COPY --from=client_builder /opt/client/build .
WORKDIR /trantor

EXPOSE 3030
CMD ["cargo", "run", "--", "config.toml"]

#FROM scratch
#WORKDIR /trantor
#COPY --from=builder /trantor/config.toml .
#COPY --from=builder /trantor/target/release/trantor .

#CMD ["./trantor"]