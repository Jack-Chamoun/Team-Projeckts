# Use rust base image
FROM rust:1.87

# set workdir
WORKDIR /app

# copy everything from current dir to workdir
COPY . .

# install all the neccessary packages
RUN rustup target add wasm32-unknown-unknown && \
cargo install --locked trunk 

EXPOSE 8080

# start weather app on trunk server
CMD ["trunk", "serve", "--address", "0.0.0.0"]