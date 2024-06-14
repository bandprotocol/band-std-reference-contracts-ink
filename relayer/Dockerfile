FROM golang:1.21.3-alpine3.18 AS build

RUN set -eux; apk add --no-cache ca-certificates build-base;

WORKDIR /app

COPY . .

RUN go mod download

# Build Relayer
RUN go build -tags muslc -o /substrate-relayer ./bin

##
## Deploy
##
FROM alpine:3.19

WORKDIR /

COPY config.json ./config/config.json

COPY --from=build /substrate-relayer /substrate-relayer

# For local only
ENTRYPOINT ["/substrate-relayer"]
