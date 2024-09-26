FROM rust:latest  AS wasm-builder
# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Set PATH for cargo and rustup
ENV PATH="/root/.cargo/bin:${PATH}"
# Install necessary tools for building wasm
RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /flai_rs

## build the simulation
#COPY libs/ ./libs/
COPY . .
RUN cd libs/simulation-wasm/ && wasm-pack build
RUN ls -l libs/simulation-wasm/pkg


FROM node:16 AS node-builder
WORKDIR /flai_rs
COPY --from=wasm-builder /flai_rs/ ./
COPY --from=wasm-builder /flai_rs/libs/simulation-wasm/pkg ./libs/simulation-wasm/pkg
WORKDIR /flai_rs/www
RUN npm install

# Expose the port 
EXPOSE 42069
# Define the command to run the app
CMD ["npm", "run", "start", "--", "--host", "0.0.0.0", "--port", "42069"]
