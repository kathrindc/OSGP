DELETE FROM logon_sessions;

ALTER TABLE logon_sessions
    ADD COLUMN address VARCHAR NOT NULL;
