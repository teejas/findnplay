FROM golang:1.21-alpine

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY *.go ./
COPY web/ web/

RUN CGO_ENABLED=0 GOOS=linux go build -o /findnplay

EXPOSE 8080

# Run
CMD ["/findnplay", "-d", "media"]

# docker run -p 8080:8080 -v ${PWD}/media:/app/media tj1997/findnplay:latest 
