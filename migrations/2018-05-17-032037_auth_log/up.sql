CREATE TABLE auth_log (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    success BOOLEAN NOT NULL,
    ip_address VARCHAR(15) NOT NULL,
    user_agent VARCHAR NOT NULL
);

CREATE INDEX ON auth_log USING hash (email);
CREATE INDEX ON auth_log USING hash (ip_address);
