# michaelfbryan.com

A crappy little website for making my life easier. Written using React (pretty
frontend), Golang (backend API), and backed by MongoDB.

## Getting Started

Make sure you have the following dependencies installed:

- Go compiler
- Node.JS & NPM
- Docker

First, start up a docker container to host our database.

```console
docker run -d -p 27017:27017 mongo
```

Next, install the website itself (`website-server`) and the administration tool
(`websitectl`).

```console
go install ./cmd/...
```

If you want to do some development, we can take advantage of
`create-react-app`'s built-in livereload by getting the website to proxy all
requests for static assets to the react dev server.

```console
$ cd frontend
$ npm start
Compiled successfully!

You can now view frontend in the browser.

  Local:            http://localhost:3000/
    On Your Network:  http://192.168.1.137:3000/

    Note that the development build is not optimized.
    To create a production build, use npm run build.

# In another terminal
$ website-server --dev http://localhost:3000/
```

You can then visit the website at http://localhost:8000/ and it'll automatically
update any time frontend code is changed.

Otherwise, for release builds:

```console
cd frontend && npm run build && cd..
website-server --host 0.0.0.0 --port 80
```

When the database is first created it will be empty, meaning it's not possible
to log in. You can create new users via `websitectl`.

```console
$ websitectl --db localhost:27017 create-user --username admin --password Password1
```
