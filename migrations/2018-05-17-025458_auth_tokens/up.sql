CREATE TABLE auth_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL
        CONSTRAINT fk_auth_tokens_user_id REFERENCES users(id),
    token BYTEA NOT NULL,
    date_created TIMESTAMP WITH TIME ZONE NOT NULL
        CONSTRAINT df_auth_tokens_date_created DEFAULT (now() AT TIME ZONE 'utc'),
    date_expired TIMESTAMP WITH TIME ZONE NOT NULL,
    token_type VARCHAR(15) NOT NULL
);

CREATE INDEX ix_auth_tokens_token ON auth_tokens USING hash (token);
