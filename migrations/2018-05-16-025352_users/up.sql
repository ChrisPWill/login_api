CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(60) NOT NULL,
    date_created TIMESTAMP WITH TIME ZONE NOT NULL
        CONSTRAINT df_users_date_created DEFAULT (now() AT TIME ZONE 'utc')
)
