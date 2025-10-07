FROM rust:1.90.0

ADD . /code
WORKDIR /code

RUN cargo install --path .
CMD ["ferenginar"]