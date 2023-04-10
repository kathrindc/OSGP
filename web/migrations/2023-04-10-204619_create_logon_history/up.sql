CREATE TABLE logon_history (
    "id" UUID NOT NULL PRIMARY KEY,
    "user" INT NOT NULL REFERENCES users ("id"),
    "address" VARCHAR NOT NULL,
    "started_at" TIMESTAMP WITH TIME ZONE NOT NULL
);
