# packet-tracer

## Prerequisites

1. stable rust toolchains: `rustup toolchain install stable`
1. nightly rust toolchains: `rustup toolchain install nightly --component rust-src`
1. (if cross-compiling) rustup target: `rustup target add ${ARCH}-unknown-linux-musl`
1. (if cross-compiling) LLVM: (e.g.) `brew install llvm` (on macOS)
1. (if cross-compiling) C toolchain: (e.g.) [`brew install filosottile/musl-cross/musl-cross`](https://github.com/FiloSottile/homebrew-musl-cross) (on macOS)
1. bpf-linker: `cargo install bpf-linker` (`--no-default-features` on macOS)

## Build & Run

Use `cargo build`, `cargo check`, etc. as normal. Run your program with:

```shell
cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

Cargo build scripts are used to automatically build the eBPF correctly and include it in the
program.

## Cross-compiling on macOS

Cross compilation should work on both Intel and Apple Silicon Macs.

```shell
CC=${ARCH}-linux-musl-gcc cargo build --package packet-tracer --release \
  --target=${ARCH}-unknown-linux-musl \
  --config=target.${ARCH}-unknown-linux-musl.linker=\"${ARCH}-linux-musl-gcc\"
```
The cross-compiled program `target/${ARCH}-unknown-linux-musl/release/packet-tracer` can be
copied to a Linux server or VM and run there.


# Maps

Note: A lot of the `Maps` section is taken from this [tutorial](https://medium.com/@stevelatif/aya-rust-tutorial-part-5-using-maps-4d26c4a2fff8)

The eBPF verifier enforces a 512 byte limit per stack frame, if you need to handle more data you can store data using Maps. Below are the different types of maps:

| Map Type                          | Description |
|----------------------------------|-------------|
| `BPF_MAP_TYPE_ARRAY`             | Fixed-size array where keys are indexes (like C arrays). |
| `BPF_MAP_TYPE_PERCPU_ARRAY`      | Like `ARRAY`, but each CPU gets its own copy for lock-free access. |
| `BPF_MAP_TYPE_PROG_ARRAY`        | Holds programs for tail calls (used for chaining eBPF programs). |
| `BPF_MAP_TYPE_PERF_EVENT_ARRAY`  | Used for pushing events to user space via perf ring buffers. |
| `BPF_MAP_TYPE_CGROUP_ARRAY`      | Array of cgroup references, used for filtering or attaching programs to cgroups. |
| `BPF_MAP_TYPE_CGROUP_STORAGE`    | Per-cgroup storage for data that persists across invocations. |
| `BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE` | Per-CPU variant of `CGROUP_STORAGE`. |
| `BPF_MAP_TYPE_HASH`              | Classic key-value hash map. |
| `BPF_MAP_TYPE_PERCPU_HASH`       | Per-CPU variant of `HASH`, allowing lock-free access. |
| `BPF_MAP_TYPE_LRU_HASH`          | Hash map with least-recently-used eviction. |
| `BPF_MAP_TYPE_LRU_PERCPU_HASH`   | Per-CPU version of `LRU_HASH`. |
| `BPF_MAP_TYPE_LPM_TRIE`          | Longest prefix match trie, useful for IP routing lookups. |
| `BPF_MAP_TYPE_STACK_TRACE`       | Stores stack traces collected by BPF programs. |
| `BPF_MAP_TYPE_ARRAY_OF_MAPS`     | Array where each element is a map (map-of-maps). |
| `BPF_MAP_TYPE_HASH_OF_MAPS`      | Hash map where each value is another map (map-of-maps). |
| `BPF_MAP_TYPE_INODE_STORAGE`     | Per-inode storage, similar to task or cgroup storage. |
| `BPF_MAP_TYPE_TASK_STORAGE`      | Storage associated with a task (process/thread). |
| `BPF_MAP_TYPE_DEVMAP`            | Used for redirecting packets to other devices in XDP. |
| `BPF_MAP_TYPE_DEVMAP_HASH`       | Hash variant of `DEVMAP` for dynamic device selection. |
| `BPF_MAP_TYPE_SK_STORAGE`        | Per-socket storage for storing context or metadata. |
| `BPF_MAP_TYPE_CPUMAP`            | Used to redirect packets to specific CPUs. |
| `BPF_MAP_TYPE_XSKMAP`            | Used with AF_XDP sockets for high-performance packet processing. |
| `BPF_MAP_TYPE_SOCKMAP`           | Stores socket references for msg/stream redirection. |
| `BPF_MAP_TYPE_SOCKHASH`          | Hash map version of `SOCKMAP` for fast lookups. |
| `BPF_MAP_TYPE_REUSEPORT_SOCKARRAY` | Select sockets from a reuseport group (SO_REUSEPORT). |
| `BPF_MAP_TYPE_QUEUE`             | FIFO queue data structure. |
| `BPF_MAP_TYPE_STACK`             | LIFO stack data structure. |
| `BPF_MAP_TYPE_STRUCT_OPS`        | Used for implementing BPF-based struct callbacks (like TCP congestion control). |
| `BPF_MAP_TYPE_RINGBUF`           | Fast, lock-free ring buffer for user space communication. |
| `BPF_MAP_TYPE_BLOOM_FILTER`      | Probabilistic data structure for set membership queries. |
| `BPF_MAP_TYPE_USER_RINGBUF`      | Similar to `RINGBUF`, but intended for user-space-allocated memory regions. |
| `BPF_MAP_TYPE_ARENA`             | Experimental map type for efficient memory allocation patterns. |
