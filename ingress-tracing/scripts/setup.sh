#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

# Display the IP address of the computer
ip route get 1.1.1.1

if [[ " $@ " =~ " setup " ]]; then
  echo "Running setup commands..."
  rustup install stable
  rustup toolchain install nightly --component rust-src
  cargo install bpf-linker
  cargo install cargo-generate
else
  echo "No setup argument detected."
fi

if [[ " $@ " =~ " build " ]]; then
  echo "Running build commands..."
  cargo generate https://github.com/aya-rs/aya-template
else
  echo "No build argument detected."
fi

if [[ " $@ " =~ " package-tcp " ]]; then
  echo "Running package-tcp commands..."
  cd test-tcp
  docker build . -t simple-tcp-server
else
  echo "No package-tcp argument detected."
fi

if [[ " $@ " =~ " run-tcp " ]]; then
  echo "Running run-tcp commands..."
  docker run --rm -it --network host simple-tcp-server
else
  echo "No run-tcp argument detected."
fi

if [[ " $@ " =~ " call-tcp " ]]; then
  echo "Running call-tcp commands..."
  cd test-tcp-client
  cargo run
else
  echo "No run-tcp argument detected."
fi

if [[ " $@ " =~ " run-docker-listener " ]]; then
  echo "Running run-docker-listener commands..."
  cd packet-tracer
  RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"' --   --iface docker0
else
  echo "No run-tcp argument detected."
fi

if [[ " $@ " =~ " run-wifi-listener-ingress " ]]; then
  echo "Running run-docker-listener-ingress commands..."
  cd ingress/ingress
  RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"' --   --iface wlp0s20f3
else
  echo "No run-tcp argument detected."
fi

if [[ " $@ " =~ " run-wifi-listener-egress " ]]; then
  echo "Running run-docker-listener-egress commands..."
  cd egress/egress
  RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"' --   --iface wlp0s20f3
else
  echo "No run-tcp argument detected."
fi
