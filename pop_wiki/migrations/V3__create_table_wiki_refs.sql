CREATE TABLE wiki_refs(
    subreddit_from VARCHAR(32) NOT NULL,
    name VARCHAR(255) NOT NULL,
    subreddit_to VARCHAR(32) NOT NULL,
    total INTEGER NOT NULL,
    PRIMARY KEY (subreddit_from, NAME, subreddit_to)
);

