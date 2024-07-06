// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod post_refs {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
        > {
            pub author: T1,
            pub subreddit_from: T2,
            pub subreddit_to: T3,
            pub is_submission: bool,
            pub created_utc: i32,
        }
        pub fn insert() -> InsertStmt {
            InsertStmt(cornucopia_async::private::Stmt::new(
                "WITH exist_author_id AS (
    SELECT
        id
    FROM
        users
    WHERE
        name = $1
),
new_author_id AS (
INSERT INTO users(name)
    SELECT
        $1
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
        name = $2
),
new_subreddit_from_id AS (
INSERT INTO subreddits(name)
    SELECT
        $2
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
            $2 AS name,
            id
        FROM
            subreddit_from_id) AS _
    WHERE
        name = $3
),
new_subreddit_to_id AS (
INSERT INTO subreddits(name)
    SELECT
        $3
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
        VALUES ($4,(
                SELECT
                    id
                FROM
                    author_id),
                $5,
(
                    SELECT
                        id
                    FROM
                        subreddit_from_id),
(
                        SELECT
                            id
                        FROM
                            subreddit_to_id))",
            ))
        }
        pub struct InsertStmt(cornucopia_async::private::Stmt);
        impl InsertStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                author: &'a T1,
                subreddit_from: &'a T2,
                subreddit_to: &'a T3,
                is_submission: &'a bool,
                created_utc: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
                        &[
                            author,
                            subreddit_from,
                            subreddit_to,
                            is_submission,
                            created_utc,
                        ],
                    )
                    .await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertParams<T1, T2, T3>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertParams<T1, T2, T3>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(
                    client,
                    &params.author,
                    &params.subreddit_from,
                    &params.subreddit_to,
                    &params.is_submission,
                    &params.created_utc,
                ))
            }
        }
    }
}
