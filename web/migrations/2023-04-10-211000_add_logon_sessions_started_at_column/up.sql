DELETE FROM logon_sessions;

ALTER TABLE logon_sessions
    ADD COLUMN started_at TIMESTAMP WITH TIME ZONE NOT NULL;
