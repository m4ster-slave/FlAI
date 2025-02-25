FROM rust:slim AS wasm-builder

# Install basic dependencies for wasm-pack
RUN apt-get update && \
    apt-get install -y curl git gcc libc6-dev && \
    rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /flai_rs

## build the simulation
COPY ./ .
RUN cd libs/simulation-wasm/ && wasm-pack build


FROM node:16-alpine AS node-builder
WORKDIR /flai_rs
COPY --from=wasm-builder /flai_rs/ ./
COPY --from=wasm-builder /flai_rs/libs/simulation-wasm/pkg ./libs/simulation-wasm/pkg

COPY www/ ./www/
WORKDIR /flai_rs/www
RUN npm ci

# Expose the port 
EXPOSE 42069
# Define the command to run the app
CMD ["npm", "run", "start", "--", "--host", "0.0.0.0", "--port", "42069"]
