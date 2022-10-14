-- Your SQL goes here
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  subject TEXT,
  content TEXT NOT NULL,
  topic_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER
)

