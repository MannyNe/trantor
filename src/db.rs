use std::{convert::Infallible, sync::Arc};

use color_eyre::{eyre, Result};
use serde::Serialize;
use sqlx::{types::chrono::NaiveDateTime, FromRow, PgPool};
use uaparser::Parser;
use warp::Filter;

use crate::utils;

pub const DEFAULT_SOURCE_ID: i32 = 1;

#[derive(Clone)]
pub struct DB {
    pool: PgPool,
}

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl DB {
    pub async fn id_from_visitor_id(&self, visitor_id: &str) -> Result<i32> {
        let rec = sqlx::query!(
            r#"SELECT id FROM visitors WHERE visitor_id = $1"#,
            visitor_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    pub async fn id_from_source_name(&self, name: &str) -> Result<i32> {
        let rec = sqlx::query!(r#"SELECT id FROM sources WHERE name = $1"#, name)
            .fetch_one(&self.pool)
            .await?;

        Ok(rec.id)
    }
}

pub struct NewVisitorData {
    visitor_id: String,
    referer: String,
    source_id: Option<i32>,
    user_agent: String,
    user_agent_parsed: serde_json::Value,
}

impl NewVisitorData {
    pub fn new(
        user_agent: String,
        referer: String,
        source_id: Option<i32>,
        ua_parser: Arc<uaparser::UserAgentParser>,
    ) -> Self {
        let user_agent_parsed = ua_parser.parse(&user_agent);
        let user_agent_parsed = serde_json::to_value(user_agent_parsed).unwrap();

        Self {
            visitor_id: utils::generate_id(),
            user_agent,
            referer,
            source_id,
            user_agent_parsed,
        }
    }

    pub fn visitor_id(&self) -> String {
        self.visitor_id.to_owned()
    }
}

#[derive(FromRow, Serialize)]
pub struct SingleVisitor {
    id: String,
    referer: String,
    os: String,
    device: String,
    browser: String,
    #[serde(with = "native_date_format")]
    created_at: NaiveDateTime,
    source_name: Option<String>,
}

impl DB {
    pub async fn create_visitor(&self, data: &NewVisitorData) -> Result<i32> {
        let rec = sqlx::query!(
            r#"INSERT INTO visitors (visitor_id, user_agent, referer, source_id, user_agent_parsed) VALUES ($1, $2, $3, $4, $5) RETURNING id"#,
            data.visitor_id,
            data.user_agent,
            data.referer,
            data.source_id,
            data.user_agent_parsed
        ).fetch_one(&self.pool).await?;

        Ok(rec.id)
    }

    pub async fn count_visitors(&self) -> Result<i64> {
        let rec = sqlx::query!(r#"SELECT COUNT(id) as count FROM visitors"#)
            .fetch_one(&self.pool)
            .await?;

        rec.count.ok_or_else(|| eyre::eyre!("No count found"))
    }

    pub async fn list_visitors(&self) -> Result<Vec<SingleVisitor>> {
        let visitors = sqlx::query_as!(
            SingleVisitor,
            r#"
            SELECT visitors.visitor_id as "id!",
                visitors.referer as "referer!",
                visitors.created_at as "created_at!",
                visitors.user_agent_parsed->'os'->>'family' AS "os!",
                visitors.user_agent_parsed->'device'->>'family' AS "device!",
                visitors.user_agent_parsed->'user_agent'->>'family' AS "browser!",
                sources.name as "source_name?"
            FROM visitors
                LEFT JOIN sources ON visitors.source_id = sources.id
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(visitors)
    }
}

pub struct NewSessionData {
    session_id: String,
    visitor_id: i32,
    start_timestamp: f64,
    title: String,
    pathname: String,
}

impl NewSessionData {
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl NewSessionData {
    pub fn new(visitor_id: i32, start_timestamp: f64, title: String, pathname: String) -> Self {
        Self {
            session_id: utils::generate_id(),
            visitor_id,
            start_timestamp,
            title,
            pathname,
        }
    }
}

impl DB {
    pub async fn create_session(&self, data: &NewSessionData) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO sessions (session_id, visitor_id, start_timestamp, title, pathname)
            VALUES ($1, $2, TO_TIMESTAMP($3), $4, $5)"#,
            data.session_id,
            data.visitor_id,
            data.start_timestamp,
            data.title,
            data.pathname
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn end_session(&self, session_id: &str, timestamp: f64) -> Result<()> {
        sqlx::query!(
            r#"UPDATE sessions SET ended_at = CURRENT_TIMESTAMP, end_timestamp = TO_TIMESTAMP($1) WHERE session_id = $2"#,
            timestamp,
            session_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_event(
        &self,
        session_id: &str,
        event_type: &str,
        event_target: &str,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO events (session_id, type, target)
            VALUES (
                (SELECT id FROM sessions WHERE session_id = $1), $2, $3
            )
            "#,
            session_id,
            event_type,
            event_target
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

impl DB {
    pub async fn create_source(&self, name: &str) -> Result<()> {
        sqlx::query!(r#"INSERT INTO sources (name) VALUES ($1)"#, name)
            .fetch_one(&self.pool)
            .await?;

        Ok(())
    }
}

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod native_date_format {
    use serde::{self, Serializer};
    use sqlx::types::chrono::NaiveDateTime;

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp_millis())
    }
}
