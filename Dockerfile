# use the Rust stable releas as base image
FROM rust:1.64.0

# switch working directory to app, creates the folder if not exists
WORKDIR /app

# install all the required system dependencies for the linking config
RUN apt update && apt install lld clang -y

# copy all files and source from context working environment to Docker image
COPY . .

# uses the offline sqlx features, will using sqlx-data.json during compile time
ENV SQLX_OFFLINE=true

# build the project with release profile for better optimization
RUN cargo build --release --bin zero2prod

# when `docker run` is executed, launch the binary
ENTRYPOINT ["./target/release/zero2prod"]
