CREATE TABLE subreddits(
    name VARCHAR(32) NOT NULL PRIMARY KEY,
    nsfw BOOLEAN,
    subscribers INTEGER,
    disconnected BOOLEAN
);

