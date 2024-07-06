CREATE TABLE post_refs(
    is_submission BOOLEAN NOT NULL,
    author INTEGER NOT NULL,
    created_utc INTEGER NOT NULL,
    subreddit_from INTEGER NOT NULL,
    subreddit_to INTEGER NOT NULL
);

