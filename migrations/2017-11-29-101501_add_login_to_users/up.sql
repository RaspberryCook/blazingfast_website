DROP TABLE users;

CREATE TABLE users (
    id INTEGER NOT NULL,
    firstname TEXT NOT NULL,
    lastname TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    CONSTRAINT unique_users UNIQUE (email)
    PRIMARY KEY(id)
);
