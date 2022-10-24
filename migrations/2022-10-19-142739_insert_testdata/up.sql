-- Your SQL goes here
INSERT INTO forums (
	id,
	name,
	position,
	is_locked ) VALUES
(1, "Example Forum 1", 0, false),
(2, "Example Forum 2", 1, false);

INSERT INTO boards (
	forum_id,
	name,
	description,
	updated_at,
	position,
	is_locked ) VALUES
( 1, "First Board", "Description for first Board", 0, 0, false),
( 2, "Second Board", "Description for second Board", 0, 1, false);

INSERT INTO users (
	id,
	name,
	about,
	picture,
	is_admin ) VALUES
("1", "user1", "I'm user 1", "https://picsum.photos/seed/user1/200/300", true),
("2", "user2", "I'm user 2", "https://picsum.photos/seed/user2/200/300", false),
("3", "user3", "I'm user 3", "https://picsum.photos/seed/user2/200/300", false);

INSERT INTO topics (
	id,
	board_id,
	user_id,
	subject,
	content,
	is_sticky,
	is_locked,
	created_at ) VALUES
(1, 1, "1", "Subject topic 1", "Content for topic 1", false, false, 0),
(2, 1, "1", "Subject topic 2", "Content for topic 2", false, false, 0),
(3, 2, "2", "Subject topic 3", "Content for topic 3", false, false, 0);


-- Your SQL goes here
INSERT INTO posts (
	id,
	user_id,
	content,
	topic_id,
	created_at
) VALUES
(1, "1", "Content for post 1", 1, 0),
(2, "2", "Content for post 2", 1, 1),
(3, "2", "Content for post 3", 2, 2),
(4, "3", "Content for post 4", 2, 3);


