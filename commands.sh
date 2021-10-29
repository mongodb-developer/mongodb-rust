# Start a container with the Rust runtimes (1.56)
docker run -it -v $(pwd)/app:/opt/app:z rust:1.56 /bin/bash

# Run the following commands in the container
cd /opt/app/rust_quickstart
cargo run