ALTER TABLE users
ADD COLUMN date_modified TIMESTAMP WITH TIME ZONE NOT NULL
        CONSTRAINT df_users_date_modified DEFAULT (now() AT TIME ZONE 'utc')
