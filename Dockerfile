FROM jimmycuadra/rust

EXPOSE 80
COPY Cargo.toml /source
COPY src/main.rs /source/src/
CMD cargo run