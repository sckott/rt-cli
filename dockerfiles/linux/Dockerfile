FROM rocker/r-base:latest

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    curl

# Update new packages
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

# Add to the path
ENV PATH="/root/.cargo/bin:${PATH}"

# Install a second version of R
RUN curl -O https://cdn.rstudio.com/r/debian-12/pkgs/r-4.3.1_1_amd64.deb && \
    apt-get install -y ./r-4.3.1_1_amd64.deb

# add alias for ll
RUN echo 'alias ll="ls -alG"' >> ~/.bashrc

ENTRYPOINT ["/bin/bash"]

# docker build -t rt:linux .
# docker run -it -v "$(pwd):/rt-cli" rt:linux
# cargo test -p rt-cli discover_linux_ -- --nocapture
