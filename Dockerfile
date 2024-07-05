FROM rust:latest

WORKDIR /server

COPY . .

RUN chmod a+x /server/entrypoint.sh

RUN cargo build --release

RUN ls /server

EXPOSE 8081

ENTRYPOINT [ "/server/entrypoint.sh" ]

CMD [ "target/release/server" ]

