###################
## BUILDER STAGE ##
###################

# use the Rust stable release as base image
FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef

# switch working directory to app, creates the folder if not exists
WORKDIR /app

# install all the required system dependencies for the linking config
RUN apt update && apt install lld clang -y


FROM chef as planner

# copy all files and source from context working environment to Docker image
COPY . .

# compute the lock-like recipe.json file
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# uses the offline sqlx features, will using sqlx-data.json during compile time
ENV SQLX_OFFLINE=true

# build the project with release profile for better optimization
RUN cargo build --release --bin zero2prod

###################
## RUNTIME STAGE ##
###################

# use the Rust stable release as base image
FROM debian:bullseye-slim AS runtime

# switch working directory to app, creates the folder if not exists
WORKDIR /app

# install required packages
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # clean up, remove not required files and packages
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# copy only the binary release target and configuration files
COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

# uses the production configuration file
ENV APP_ENVIRONMENT=production

# when `docker run` is executed, launch the binary
ENTRYPOINT ["./zero2prod"]
