# Use an official Ubuntu image as the base
FROM ubuntu:latest

# Install necessary dependencies for Rust and C++ compilation
#RUN apt-get update && apt-get install -y \
#    curl \
#    build-essential \
#    cmake \
#    gcc \
#    g++ \
#    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup (recommended for getting the latest stable version)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory inside the container
WORKDIR /docker_mlcs 

# Copy all the project files into the container
COPY . .
