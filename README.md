A simple login API server written in Rust using the Rouille web framework, Diesel API and
PostgreSQL database.

While this is currently not used in a production environment, it should provide a good starting
point onto which a project could be built. Alternatively, appropriately configured, it can act as
an independent login microservice which may be a beneficial architecture to adopt in certain
circumstances.

Example requests
================
Create user
-----------
http://localhost:8000/v1/user `POST`

Headers:

    Content-Type: application/json

Body:

    {
        "email": "hunter@test.com",
        "password": "hunter2"
    }

Example response:

    {
        "id": 2,
        "email": "hunter@test.com",
        "date_created": "2019-08-12T23:55:13.965004Z"
    }

Login
-----
http://localhost:8000/v1/user/login `POST`

Headers:

    Content-Type: application/json

Body:

    {
        "email": "hunter@test.com",
        "password": "hunter2"
    }

Example response:

    {
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoyLCJlbWFpbCI6ImNocmlzcHdpbGwrMUBnbWFpbC5jb20iLCJ0b2tlbiI6IjFhMTk3M2ZmLTdlNDQtNDFlZi04OTE0LTgyMzNmNGRhNjY4NiJ9.T9rLk2xOju93pAQxbaXnKu_RVfEdaR-9n9SUQ2T7IB4"
    }

Planned features
================
- Token validation
- MFA support (start with TOTP)

Development Setup
=================

- Install postgresql (or DB of choice - migration files will need to be modified)
- Install [diesel-cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)
- Copy .env.example to .env and configure with DB credentials and secrets for `HMAC\_HASH` and `JWT\_SECRET`
  - Note: In a production environment, secrets and DB strings should not be configured via a file
- Run the following commands to set up the DB:
```
diesel setup
diesel migration run
```
- Run the dev server with:
```
cargo run
```

Formatting Code
---------------

Note: Currently the nightly version of rustfmt is used for formatting purposes only. The project
itself is written and tested on the most current version of stable Rust.

To install rustfmt:
```
rustup component add rustfmt --toolchain nightly
```
To format the code:
```
cargo +nightly fmt
```

License
=======
MIT License, see LICENSE
