PRAGMA foreign_keys=off;

BEGIN TRANSACTION;

ALTER TABLE users RENAME TO users_temp;

CREATE TABLE users (
    id INTEGER NOT NULL,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    PRIMARY KEY(id)
);

INSERT INTO users SELECT * FROM users_temp;

COMMIT;

PRAGMA foreign_keys=on;