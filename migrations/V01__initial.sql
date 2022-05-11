CREATE TABLE notes (
    user_id INTEGER,
    twitter_account_id INTEGER,
    text TEXT NOT NULL,
    PRIMARY KEY (user_id, twitter_account_id)
);
