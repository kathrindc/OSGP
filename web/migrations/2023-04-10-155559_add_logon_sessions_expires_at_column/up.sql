DELETE FROM logon_sessions;

ALTER TABLE logon_sessions
    ADD COLUMN expires_at TIMESTAMP WITH TIME ZONE NOT NULL;
