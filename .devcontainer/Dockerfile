FROM gcc:latest

RUN apt-get update && \
    apt-get install -y \ 
    build-essential \
    gdb \
    cmake \
    zsh \
    fish \
    tmux \
    python3 \
    python3-pip \
    curl \
    wget \
    ca-certificates \
    xz-utils && \
    curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    #Install Jasmine
    npm install -g jasmine && \
    # Install Rust
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    # Install Go
    GO_VERSION=1.22.3 && \
    wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz && \
    rm -rf /usr/local/go && \
    tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz && \
    rm go${GO_VERSION}.linux-amd64.tar.gz && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set up environment variables for Rust and Go
ENV PATH="/root/.cargo/bin:/usr/local/go/bin:${PATH}"
