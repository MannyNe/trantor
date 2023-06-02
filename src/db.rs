use std::{convert::Infallible, sync::Arc};

use color_eyre::{eyre, Result};
use serde::Serialize;
use sqlx::{
    postgres::types::PgInterval,
    types::{chrono::NaiveDateTime, BigDecimal},
    FromRow, PgPool,
};
use uaparser::Parser;
use warp::Filter;

use crate::utils;

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

    pub async fn id_from_tracking_id(&self, tracking_id: &str) -> Result<i32> {
        log::info!("Extracting tracking id: {:?}", tracking_id);

        let rec = sqlx::query!(
            r#"SELECT id FROM trackings WHERE tracking_id = $1"#,
            tracking_id
        )
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
    tracking_id: i32,
}

impl NewVisitorData {
    pub fn new(
        user_agent: String,
        referer: String,
        source_id: Option<i32>,
        ua_parser: Arc<uaparser::UserAgentParser>,
        tracking_id: i32,
    ) -> Self {
        let user_agent_parsed = ua_parser.parse(&user_agent);
        let user_agent_parsed = serde_json::to_value(user_agent_parsed).unwrap();

        Self {
            visitor_id: utils::generate_id(),
            user_agent,
            referer,
            source_id,
            user_agent_parsed,
            tracking_id,
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
            r#"INSERT INTO visitors (
                visitor_id, user_agent, referer, source_id, user_agent_parsed, tracking_id
            ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"#,
            data.visitor_id,
            data.user_agent,
            data.referer,
            data.source_id,
            data.user_agent_parsed,
            data.tracking_id
        )
        .fetch_one(&self.pool)
        .await?;

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

    pub async fn count_visitors_without_source(&self) -> Result<i64> {
        let rec =
            sqlx::query!(r#"SELECT COUNT(id) as count FROM visitors WHERE source_id IS NULL"#)
                .fetch_one(&self.pool)
                .await?;

        rec.count.ok_or_else(|| eyre::eyre!("No count found"))
    }
}

pub struct NewSessionData {
    session_id: String,
    visitor_id: i32,
    start_timestamp: f64,
    title: String,
    pathname: String,
    tracking_id: i32,
}

impl NewSessionData {
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl NewSessionData {
    pub fn new(
        visitor_id: i32,
        start_timestamp: f64,
        title: String,
        pathname: String,
        tracking_id: i32,
    ) -> Self {
        Self {
            session_id: utils::generate_id(),
            visitor_id,
            start_timestamp,
            title,
            pathname,
            tracking_id,
        }
    }
}

#[derive(FromRow, Serialize)]
pub struct SingleSession {
    id: String,
    title: String,
    pathname: String,
    #[serde(with = "native_date_format")]
    start_timestamp: NaiveDateTime,
    #[serde(with = "optional_native_date_format")]
    end_timestamp: Option<NaiveDateTime>,
    #[serde(with = "optional_pg_interval_format")]
    start_latency: Option<PgInterval>,
    #[serde(with = "optional_pg_interval_format")]
    end_latency: Option<PgInterval>,
}

#[derive(FromRow, Serialize)]
pub struct SessionCountByWeekday {
    #[serde(with = "big_decimal_to_weekday")]
    weekday: Option<BigDecimal>,
    count: Option<i64>,
}

impl DB {
    pub async fn create_session(&self, data: &NewSessionData) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO sessions (session_id, visitor_id, start_timestamp, title, pathname, tracking_id)
            VALUES ($1, $2, TO_TIMESTAMP($3), $4, $5, $6)"#,
            data.session_id,
            data.visitor_id,
            data.start_timestamp,
            data.title,
            data.pathname,
            data.tracking_id
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
        tracking_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO events (session_id, type, target, tracking_id)
            VALUES (
                (SELECT id FROM sessions WHERE session_id = $1), $2, $3, $4
            )
            "#,
            session_id,
            event_type,
            event_target,
            tracking_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_sessions(&self) -> Result<Vec<SingleSession>> {
        let sessions = sqlx::query_as!(
            SingleSession,
            r#"
            SELECT session_id as id,
                title,
                pathname,
                start_timestamp,
                end_timestamp,
                created_at - start_timestamp as start_latency,
                ended_at - end_timestamp as end_latency
            FROM sessions
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    pub async fn count_sessions(&self) -> Result<i64> {
        let rec = sqlx::query!(r#"SELECT COUNT(id) as count FROM sessions"#)
            .fetch_one(&self.pool)
            .await?;

        rec.count.ok_or_else(|| eyre::eyre!("No count found"))
    }

    pub async fn count_sessions_by_weekday(
        &self,
        tracking_id: i32,
    ) -> Result<Vec<SessionCountByWeekday>> {
        let rec = sqlx::query_as!(
            SessionCountByWeekday,
            r#"
            SELECT COUNT(id) as count,
                EXTRACT(DOW FROM start_timestamp) as weekday
            FROM sessions
            WHERE tracking_id = $1
            GROUP BY weekday
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }
}

#[derive(FromRow, Serialize)]
pub struct SingleSource {
    name: String,
    visitor_count: i64,
}

impl DB {
    pub async fn create_source(&self, name: &str) -> Result<()> {
        let _ = sqlx::query!(
            r#"INSERT INTO sources (name) VALUES ($1) RETURNING id"#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_sources(&self) -> Result<Vec<SingleSource>> {
        let sources = sqlx::query_as!(
            SingleSource,
            r#"
            SELECT sources.name as "name!",
                COUNT(visitors.id) as "visitor_count!"
            FROM sources
                LEFT JOIN visitors ON visitors.source_id = sources.id
            GROUP BY sources.name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sources)
    }

    pub async fn count_sources(&self) -> Result<i64> {
        let rec = sqlx::query!(r#"SELECT COUNT(id) as count FROM sources"#)
            .fetch_one(&self.pool)
            .await?;

        rec.count.ok_or_else(|| eyre::eyre!("No count found"))
    }
}

pub struct NewUserData {
    user_id: String,
    secret_code: String,
}

impl NewUserData {
    pub fn new(secret_code: &str) -> Self {
        Self {
            user_id: utils::generate_id(),
            secret_code: secret_code.to_owned(),
        }
    }
}

#[derive(FromRow, Serialize)]
pub struct CreatedUser {
    user_id: String,
    secret_code: String,
}

impl DB {
    pub async fn create_user(&self, data: &NewUserData) -> Result<CreatedUser> {
        let user = sqlx::query_as!(
            CreatedUser,
            r#"INSERT INTO users (user_id, secret_code) VALUES ($1, $2) RETURNING user_id, secret_code"#,
            data.user_id,
            data.secret_code
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn authenticate_user(&self, user_id: &str) -> Result<(i32, String)> {
        let rec = sqlx::query!(
            r#"SELECT id, secret_code FROM users WHERE user_id = $1"#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((rec.id, rec.secret_code))
    }
}

pub struct NewTrackingData {
    tracking_id: String,
    name: String,
    owner_id: i32,
}

impl NewTrackingData {
    pub fn new(name: String, owner_id: i32) -> Self {
        Self {
            tracking_id: utils::generate_id(),
            name,
            owner_id,
        }
    }
}

#[derive(FromRow, Serialize)]
pub struct SingleTracking {
    id: String,
    name: String,
    #[serde(with = "native_date_format")]
    created_at: NaiveDateTime,
    visitor_count: Option<i64>,
    sessions_count: Option<i64>,
    events_count: Option<i64>,
    sources_count: Option<i64>,
}

impl DB {
    pub async fn create_tracking(&self, data: &NewTrackingData) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO trackings (tracking_id, name, owner_id) VALUES ($1, $2, $3)"#,
            data.tracking_id,
            data.name,
            data.owner_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_trackings(&self, owner_id: i32) -> Result<Vec<SingleTracking>> {
        let trackings = sqlx::query_as!(
            SingleTracking,
            r#"
            SELECT trackings.tracking_id as id,
                trackings.name as name,
                trackings.created_at as created_at,
                COUNT(DISTINCT visitors.id) as visitor_count,
                COUNT(DISTINCT sessions.id) as sessions_count,
                COUNT(DISTINCT events.id) as events_count,
                COUNT(DISTINCT sources.id) as sources_count
            FROM trackings
                LEFT JOIN visitors ON visitors.tracking_id = trackings.id
                LEFT JOIN sessions ON sessions.tracking_id = trackings.id
                LEFT JOIN events ON events.tracking_id = trackings.id
                LEFT JOIN sources ON sources.tracking_id = trackings.id
            WHERE trackings.owner_id = $1 
            GROUP BY trackings.tracking_id, trackings.name, trackings.created_at
        "#,
            owner_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(trackings)
    }

    pub async fn tracking_owner(&self, tracking_id: &str) -> Result<i32> {
        let rec = sqlx::query!(
            r#"SELECT owner_id FROM trackings WHERE tracking_id = $1"#,
            tracking_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.owner_id)
    }
}

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

mod native_date_format {
    use serde::{self, Serializer};
    use sqlx::types::chrono::NaiveDateTime;

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp_millis())
    }
}

mod optional_native_date_format {
    use serde::{self, Serializer};
    use sqlx::types::chrono::NaiveDateTime;

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => serializer.serialize_i64(date.timestamp_millis()),
            None => serializer.serialize_none(),
        }
    }
}

mod optional_pg_interval_format {
    use serde::{self, Serializer};
    use sqlx::postgres::types::PgInterval;

    pub fn serialize<S>(interval: &Option<PgInterval>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match interval {
            Some(interval) => serializer.serialize_some(&serde_json::json!({
                "months": interval.months,
                "days": interval.days,
                "microseconds": interval.microseconds,
            })),
            None => serializer.serialize_none(),
        }
    }
}

mod big_decimal_to_weekday {
    use num_traits::cast::ToPrimitive;
    use serde::{self, Serializer};
    use sqlx::types::BigDecimal;

    pub fn serialize<S>(decimal: &Option<BigDecimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // this is safe because the value is always between 0 and 6
        match decimal {
            Some(decimal) => match decimal.to_u8() {
                Some(value) => serializer.serialize_u8(value),
                None => serializer.serialize_none(),
            },
            None => serializer.serialize_none(),
        }
    }
}
