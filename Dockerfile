FROM rust:latest

WORKDIR /server

COPY . .

RUN chmod a+x /server/entrypoint.sh

RUN ls -la

RUN cargo build --release

EXPOSE 80

ENTRYPOINT [ "/server/entrypoint.sh" ]

CMD [ "--server.port=80" ]

