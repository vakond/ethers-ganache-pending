Rust toolchain: 
---------------
1.47.0-x86_64-unknown-linux-gnu

In directory of the project:
----------------------------
    docker build --force-rm --file Dockerfile.ganache --tag ganache .
    docker run --detach --name ganache0 --publish 8545:8545 ganache
    cargo run --release

Output:
-------
    Pending the transaction...
    thread 'tokio-runtime-worker' panicked at 'WS Server panic: NoResponse'
    or just hangs if use the Http-transport.
