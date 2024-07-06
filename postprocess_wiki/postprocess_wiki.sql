-- Compute references
INSERT INTO wiki_refs(subreddit_from, name, subreddit_to, total)
SELECT
    subreddit AS subreddit_from,
    name,
    lower(re_match[1]) AS subreddit_to,
    count(*) AS total
FROM
    wikis,
    LATERAL regexp_matches(content, '[^A-Za-z0-9_\-]r\/([A-Za-z0-9_\-]{1,20})[^A-Za-z0-9_\-]', 'g') AS re_match
GROUP BY
    subreddit,
    name,
    lower(re_match[1]);

-- Check disconnected subreddits
UPDATE
    subreddits
SET
    disconnected = FALSE;

WITH selfless_refs AS (
    SELECT
        *
    FROM
        wiki_refs
    WHERE
        subreddit_from <> subreddit_to
),
disconnected_subreddits AS (
    SELECT
        subreddits.name
    FROM
        subreddits
        LEFT JOIN selfless_refs AS wr0 ON wr0.subreddit_from = subreddits.name
        LEFT JOIN selfless_refs AS wr1 ON wr1.subreddit_to = subreddits.name
    WHERE
        wr0.subreddit_from IS NULL
        AND wr1.subreddit_to IS NULL)
UPDATE
    subreddits
SET
    disconnected = TRUE
WHERE
    name IN (
        SELECT
            name
        FROM
            disconnected_subreddits);
