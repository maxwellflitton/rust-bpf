FROM ubuntu:22.04

# Setup environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV LLVM_VERSION=15
ENV RUST_VERSION=1.76.0

# Install system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    clang \
    llvm-$LLVM_VERSION \
    llvm-$LLVM_VERSION-dev \
    libclang-common-$LLVM_VERSION-dev \
    libclang-$LLVM_VERSION-dev \
    libclang-cpp$LLVM_VERSION-dev \
    zlib1g-dev \
    curl \
    git \
    pkg-config \
    python3 \
    ca-certificates \
    sudo \
    iproute2 \
    linux-tools-common \
    linux-tools-generic \
    libelf-dev \
    make \
    wget \
    vim \
    unzip \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Install Rust (stable and nightly)
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup install stable
RUN rustup toolchain install nightly --component rust-src

# Install cargo tools
RUN cargo install cargo-generate

# Install bpf-linker (linking to LLVM installed above)
ENV LLVM_SYS_150_PREFIX=/usr/lib/llvm-$LLVM_VERSION
RUN LLVM_SYS_150_PREFIX=$LLVM_SYS_150_PREFIX cargo install --no-default-features bpf-linker

# Install bpftool (manual build from source because distros are usually out-of-date)
RUN git clone --depth=1 --branch v6.7 https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git /src/linux && \
    cd /src/linux/tools/bpf/bpftool && \
    make -j$(nproc) && \
    make install && \
    cd / && rm -rf /src/linux

# Set working directory (where your code lives in container)
WORKDIR /workspace

# Expose a volume so you can mount your code
VOLUME ["/workspace"]

CMD ["/bin/bash"]
