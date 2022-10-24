-- Your SQL goes here
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  content TEXT NOT NULL,
  topic_id INTEGER NOT NULL,
  created_at INTEGER NOT NULL
)

