ALTER TABLE auth_log
ADD COLUMN date_created TIMESTAMP WITH TIME ZONE NOT NULL
    CONSTRAINT df_auth_log_date_created DEFAULT (now() AT TIME ZONE 'utc');
