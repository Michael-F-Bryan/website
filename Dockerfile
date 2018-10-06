FROM golang:1.8

WORKDIR /go/src/app
COPY ./website-server .
COPY ./frontend/build ./static

EXPOSE 8000
ENV DATABASE_URL="mongodb://mongo:27017"

CMD ["/go/src/app/website-server", \
     "--entry", "./static/index.html", \
     "--static", "./static", \
     "--host", "0.0.0.0", \
     "--port", "8000"]
