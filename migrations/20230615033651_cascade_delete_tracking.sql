ALTER TABLE visitors DROP CONSTRAINT visitors_tracking_id_visitor_id_key;
ALTER TABLE sessions DROP CONSTRAINT sessions_tracking_id_session_id_key;

ALTER TABLE visitors
ADD CONSTRAINT visitors_unique_tracking_id_visitor_id UNIQUE (id, tracking_id);
ALTER TABLE sessions
ADD CONSTRAINT sessions_unique_tracking_id_session_id UNIQUE (id, tracking_id);
ALTER TABLE events
ADD CONSTRAINT events_unique_tracking_id_session_id UNIQUE (id, tracking_id, session_id);

ALTER TABLE visitors DROP CONSTRAINT visitors_tracking_id_fkey;
ALTER TABLE sessions DROP CONSTRAINT sessions_tracking_id_fkey;
ALTER TABLE sources DROP CONSTRAINT sources_tracking_id_fkey;
ALTER TABLE events DROP CONSTRAINT events_tracking_id_fkey;

ALTER TABLE visitors
ADD CONSTRAINT fk_visitors_trackings FOREIGN KEY (tracking_id) REFERENCES trackings(id) ON DELETE CASCADE;
ALTER TABLE sessions
ADD CONSTRAINT fk_sessions_trackings FOREIGN KEY (tracking_id) REFERENCES trackings(id) ON DELETE CASCADE;
ALTER TABLE sources
ADD CONSTRAINT fk_sources_trackings FOREIGN KEY (tracking_id) REFERENCES trackings(id) ON DELETE CASCADE;
ALTER TABLE events
ADD CONSTRAINT fk_events_trackings FOREIGN KEY (tracking_id) REFERENCES trackings(id) ON DELETE CASCADE;