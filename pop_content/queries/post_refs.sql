--! insert (author, subreddit_from, subreddit_to, is_submission, created_utc)
WITH exist_author_id AS (
    SELECT
        id
    FROM
        users
    WHERE
        name = :author
),
new_author_id AS (
INSERT INTO users(name)
    SELECT
        :author
    WHERE
        NOT EXISTS (
            SELECT
                *
            FROM
                exist_author_id)
        RETURNING
            id
),
author_id AS ((
        SELECT
            id
        FROM
            exist_author_id)
    UNION
    SELECT
        id
    FROM
        new_author_id
),
exist_subreddit_from_id AS (
    SELECT
        id
    FROM
        subreddits
    WHERE
        name = :subreddit_from
),
new_subreddit_from_id AS (
INSERT INTO subreddits(name)
    SELECT
        :subreddit_from
    WHERE
        NOT EXISTS (
            SELECT
                *
            FROM
                exist_subreddit_from_id)
        RETURNING
            id
),
subreddit_from_id AS ((
        SELECT
            id
        FROM
            exist_subreddit_from_id)
    UNION
    SELECT
        id
    FROM
        new_subreddit_from_id
),
exist_subreddit_to_id AS (
    SELECT
        id
    FROM (
        SELECT
            name,
            id
        FROM
            subreddits
        UNION
        SELECT
            :subreddit_from AS name,
            id
        FROM
            subreddit_from_id) AS _
    WHERE
        name = :subreddit_to
),
new_subreddit_to_id AS (
INSERT INTO subreddits(name)
    SELECT
        :subreddit_to
    WHERE
        NOT EXISTS (
            SELECT
                *
            FROM
                exist_subreddit_to_id)
        RETURNING
            id
),
subreddit_to_id AS ((
        SELECT
            id
        FROM
            exist_subreddit_to_id)
    UNION
    SELECT
        id
    FROM
        new_subreddit_to_id)
    INSERT INTO post_refs(is_submission, author, created_utc, subreddit_from, subreddit_to)
        VALUES (:is_submission,(
                SELECT
                    id
                FROM
                    author_id),
                :created_utc,
(
                    SELECT
                        id
                    FROM
                        subreddit_from_id),
(
                        SELECT
                            id
                        FROM
                            subreddit_to_id));

