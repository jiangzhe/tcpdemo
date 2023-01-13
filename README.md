## Benchmark performance of TCP single-byte transferring

- build

```bash
cargo build --release
cd go
go build ./cmd/gocli/gocli.go
go build ./cmd/gosvr/gosvr.go
```

- run

Start server in one terminal:

```
# start tokio server
target/release/svr

# or start smol server
# target/release/smolsvr

# or start go server
# go/gosvr
```

Start client in another terminal:

```
# start tokio client
target/release/cli

# or start smol client
# target/release/smolcli

# or start go client
# go/gocli
```

The server terminal will output total requests, duration and QPS once client stops.

- flamegraph

```bash
cargo flamegraph --bin cli
cargo flamegraph --bin smolcli
```

You can find tokio client has background thread requiring additional locks that cost extra CPUs.
