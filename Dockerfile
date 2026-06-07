# syntax=docker/dockerfile:1.24
# check=error=true

FROM rust:1.96-slim-trixie AS build
WORKDIR /app
COPY --exclude=target --exclude=.git . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \                
    --mount=type=cache,target=/app/target \                              
    cargo build --release && \                                           
    cp target/release/rust-template /rust-template                       

FROM gcr.io/distroless/cc-debian13:nonroot
COPY --link --from=build /rust-template /
ENTRYPOINT ["/rust-template"]
