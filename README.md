# My Website

A simple website to help make my life easier.


## Overall Website Structure

- `/times` - Overall timesheet tracker for the logged in user (shows all 
  available entries and time slices)
  - `/times/entry` - endpoints corresponding to timesheet entries
    - `/new` - Create a new timesheet entry
    - `/:n` - Show the `n`'th timesheet entry
    - `/:n/edit` - Edit the `n`'th timesheet entry
    - `/download` - Downloads a JSON blob of all available timesheet entries
  - `/times/slice` - endpoints corresponding to time slices (a collection of 
    entries between two points in time)
    - `/new` - Create a new unique *time slice*
    - `/:hash` - Show the *time slice* with the corresponding `hash`
    - `/:hash/edit` - Edit the *time slice* with the corresponding `hash`
- `/resume` - My resume
- `/login` - Allows a user to log into the site
- `/logout` - Log out
- `/admin` - The admin area (for managing users, jobs, and backups)


## Getting Started

This website uses [rocket] so you'll need to have the latest nightly installed.

```
$ rustup default nightly
```

### Running Locally

You need to have a postgres database running on your local machine.

```text
$ docker run -d --rm -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres
```

First you'll want to set a `DATABASE_URL` environment variable so all our tools
know which database you want to talk to. For example, to use a local postgres
instance (user: `postgres`, password: `postgres`):

```text
$ export DATABASE_URL=postgres://postgres:postgres@localhost/website
```

Then apply the various migrations.

```text
$ diesel setup database
```

> Note: Make sure you have the `diesel` CLI tool installed 
> (`cargo install diesel_cli --no-default-features --features postgres`).

Now your database and everything is set up, but it doesn't have any users yet!
The `website-cli` program lets you interact directly with the database for doing
routine admin tasks.

```text
$ cargo run -p website-cli -- create-user --admin michael password
```


[rocket]: https://rocket.rs/