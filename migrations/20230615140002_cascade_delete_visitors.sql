ALTER TABLE sessions DROP CONSTRAINT sessions_visitor_id_fkey;

ALTER TABLE sessions
ADD CONSTRAINT fk_sessions_visitors FOREIGN KEY (visitor_id) REFERENCES visitors(id) ON DELETE CASCADE;