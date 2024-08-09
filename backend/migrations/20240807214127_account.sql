CREATE TABLE IF NOT EXISTS account (
    user_id             CHAR(32) PRIMARY KEY,
    mail_addr           CHAR(128) NOT NULL UNIQUE,
    hashed_password     CHAR(128) NOT NULL,
    salt                CHAR(32) NOT NULL,
    user_name           CHAR(256) NOT NULL
);