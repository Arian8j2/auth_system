-- TODO: make code string instead of integer
CREATE TABLE IF NOT EXISTS email_codes (
    email_address VARCHAR(64) PRIMARY KEY NOT NULL,
    last_sent_code UNSIGNED INT NOT NULL,
    last_sent_date VARCHAR(32) NOT NULL
)
