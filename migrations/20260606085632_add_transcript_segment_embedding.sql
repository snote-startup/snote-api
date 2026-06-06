CREATE EXTENSION IF NOT EXISTS vector;

ALTER TABLE transcript_segments 
ADD COLUMN embedding vector(3072);
