CREATE TABLE docker_hosts (
    "id" UUID PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "connection_type" VARCHAR NOT NULL,
    "connection_address" VARCHAR NOT NULL
);
