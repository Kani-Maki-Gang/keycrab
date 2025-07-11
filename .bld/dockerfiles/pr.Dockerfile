FROM docker.io/library/rust:1.88-bookworm

LABEL org.opencontainers.image.source=https://github.com/Kani-Maki-Gang/keycrab
LABEL org.opencontainers.image.description="The container image for building and testing the Keycrab project."

# Add dependencies
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y pkg-config ca-certificates libgpg-error-dev libgpgme-dev linux-libc-dev libc6 libc6-dev

# Adding rustfmt
RUN rustup component add rustfmt

# Adding clippy
RUN rustup component add clippy

# Adding Trunk
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk -y

# Adding node and npm
RUN curl -fsSL https://deb.nodesource.com/setup_22.x -o nodesource_setup.sh
RUN bash nodesource_setup.sh
RUN apt-get install -y nodejs

# Adding additional targets
RUN rustup target add wasm32-unknown-unknown
