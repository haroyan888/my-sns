-- Add migration script here
CREATE TABLE article (
	article_id	CHAR(32) PRIMARY KEY,
	-- user_id			CHAR(32) FOREIGN KEY,
	body				TEXT NOT NULL,
	post_date		DATETIME DEFAULT CURRENT_TIMESTAMP
);