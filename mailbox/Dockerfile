FROM rust:1.68.1 as build
# checkout https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# create a new empty shell project
RUN USER=root cargo new --bin mailbox
WORKDIR /mailbox

COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/mailbox*
RUN cargo build --release

# our final base
FROM rust:1.68.1

# copy the build artifact from the build stage
COPY --from=build /mailbox/target/release/mailbox .

# set the startup command to run your binary
CMD ["./mailbox"]