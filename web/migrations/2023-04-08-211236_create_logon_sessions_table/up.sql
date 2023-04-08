CREATE TABLE logon_sessions (
    "id" UUID PRIMARY KEY,
    "user" INT NOT NULL REFERENCES users ("id")
);
