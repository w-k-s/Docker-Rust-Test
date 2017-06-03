FROM jimmycuadra/rust

EXPOSE 8080
COPY Cargo.toml /source
COPY src/main.rs /source/src/
COPY src/services/ /source/src/services/
COPY src/models/ /source/src/models/
CMD cargo run