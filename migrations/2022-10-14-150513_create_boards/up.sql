-- Your SQL goes here
CREATE TABLE boards(
  id INTEGER NOT NULL PRIMARY KEY,
  forum_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  updated_at DATETIME NOT NULL,
  position INTEGER NOT NULL,
  is_locked BOOLEAN NOT NULL DEFAULT FALSE
)

