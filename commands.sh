# Start a container with the Rust runtimes (1.56)
docker run -it -v $(pwd)/app:/opt/app:z rust:1.56 /bin/bash

# Create the rust application template (only once)
cd /opt/app
cargo new --bin rust_quickstart

# Run the following commands in the container to compile and run the application
cd /opt/app/rust_quickstart
cargo run
