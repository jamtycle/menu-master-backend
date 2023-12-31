FROM rust:1.69.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=27040
ENV ROCKET_ENV=production

WORKDIR /app

COPY . .
RUN rustup default nightly
RUN cargo build --release

CMD ["cargo", "run"]