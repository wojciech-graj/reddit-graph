import argparse

import praw
import psycopg


parser = argparse.ArgumentParser(prog="pop_wiki")
parser.add_argument("--db", required=True)
parser.add_argument("--clean", default=True)
parser.add_argument("-c", "--client-id", required=True)
parser.add_argument("-s", "--client-secret", required=True)
parser.add_argument("-u", "--username", required=True)
parser.add_argument("-p", "--password", required=True)
args = parser.parse_args()

reddit = praw.Reddit(
    client_id=args.client_id,
    client_secret=args.client_secret,
    username=args.username,
    password=args.password,
    user_agent="linux:io.github.wojciech-graj:v0.0.1 (by /u/wojtek-graj)",
)

SELECT_ALL = "SELECT name FROM subreddits"
SELECT_NEW = "SELECT DISTINCT subreddit_to AS name FROM wiki_refs LEFT JOIN subreddits on wiki_refs.subreddit_to = subreddits.name WHERE subreddits.name IS NULL"
INSERT_SUBREDDIT = "INSERT INTO subreddits(name) VALUES (%s)"
UPDATE_SUBREDDIT = "UPDATE subreddits SET nsfw = %s, subscribers = %s WHERE name = %s"
INSERT_WIKI = "INSERT INTO wikis(subreddit, name, content) VALUES (%s, %s, %s)"

with psycopg.connect(args.db, autocommit=True) as conn:
    with conn.cursor() as cur:
        data = conn.execute(SELECT_ALL if args.clean else SELECT_NEW)
        for subreddit in data:
            try:
                name = subreddit[0]
                if not args.clean:
                    cur.execute(INSERT_SUBREDDIT, (name, ))

                subreddit = reddit.subreddit(name)
                nsfw = subreddit.over18
                subscribers = subreddit.subscribers

                cur.execute(UPDATE_SUBREDDIT, (nsfw, subscribers, name))

                wiki = subreddit.wiki
                for i, page in enumerate(wiki):
                    page_name = page.name
                    if page_name == "config/stylesheet":
                        continue
                    content = page.content_md
                    cur.execute(INSERT_WIKI, (name, page_name, content))

            except Exception as e:
                print(f"Error: {e}")
