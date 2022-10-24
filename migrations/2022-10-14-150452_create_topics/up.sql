-- Your SQL goes here
CREATE TABLE topics (
  id INTEGER NOT NULL PRIMARY KEY,
  board_id INTEGER NOT NULL,
  user_id TEXT NOT NULL,
  subject TEXT NOT NULL,
  content TEXT NOT NULL,
  is_sticky BOOLEAN NOT NULL DEFAULT FALSE,
  is_locked BOOLEAN NOT NULL DEFAULT FALSE,
  created_at INTEGER NOT NULL
)

