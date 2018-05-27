A simple login API server written in Rust using the Rouille web framework, Diesel API and PostgreSQL database.

Planned features
================
- Some sort of easy method of validating that the token has been provided in the header.
- Token expiry dates

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
To install rustfmt:
```
rustup component add rustfmt-preview
```
To format the code:
```
cargo fmt
```

License
=======
MIT License, see LICENSE
