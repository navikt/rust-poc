FROM gcr.io/distroless/cc

COPY ./rust-poc .

EXPOSE 8000
ENV RUST_LOG=warn
CMD ["./rust-poc"]