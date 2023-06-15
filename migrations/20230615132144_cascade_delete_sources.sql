ALTER TABLE visitors DROP CONSTRAINT visitors_source_id_fkey;

ALTER TABLE visitors
ADD CONSTRAINT fk_visitors_sources FOREIGN KEY (source_id) REFERENCES sources(id) ON DELETE CASCADE;