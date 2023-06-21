FROM clux/muslrust:1.70.0 AS build
ARG OPERATOR=nodescript-operator

WORKDIR /src

COPY . .

RUN \
	mkdir -p /cargo/cargo && \
	ln -sf $HOME/.cargo/config /cargo/cargo && \
	CARGO_HOME=/cargo/cargo \
	CARGO_TARGET_DIR=/cargo/target \
	cargo install \
		--path . \
		--root /app \
		--bin $OPERATOR

FROM gcr.io/distroless/static:nonroot

COPY --from=build /app/bin/$OPERATOR /app

EXPOSE 8080

ENTRYPOINT ["/app"]
