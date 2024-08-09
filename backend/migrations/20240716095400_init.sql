CREATE TABLE IF NOT EXISTS article (
	article_id	CHAR(32) PRIMARY KEY,
	-- user_id	CHAR(32) FOREIGN KEY,
	body		TEXT NOT NULL,
	post_date	DATETIME DEFAULT (DATETIME('now', 'localtime'))
);
