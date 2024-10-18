FROM docker.io/library/rust:1.82-alpine@sha256:466dc9924d265455aa73e72fd9cdac9db69ce6a988e6f0e6baf852db3485d97d AS build

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release && ls target && cp ./target/release/secbench /bin/secbench

FROM scratch AS final
ENTRYPOINT ["/bin/secbench"]
COPY --from=build /bin/secbench /bin/secbench
