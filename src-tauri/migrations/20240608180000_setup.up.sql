CREATE TABLE greeting (
    id INTEGER PRIMARY KEY NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    message TEXT NOT NULL
);