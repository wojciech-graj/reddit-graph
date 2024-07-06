import argparse

import psycopg


parser = argparse.ArgumentParser(prog="pop_subreddits")
parser.add_argument("-i", "--input", required=True)
parser.add_argument("--db", required=True)
args = parser.parse_args()

data = []
with open(args.input, encoding="utf-8") as f:
    for line in f:
        name, size_str = line.strip().split('\t')
        size = int(size_str)
        if size >= 100:
            data.append((name, ))

with psycopg.connect(args.db) as conn:
    with conn.cursor() as cur:
        QUERY = """
        INSERT INTO subreddits (name)
        VALUES (%s)
        """
        cur.executemany(QUERY, data)
        conn.commit()
