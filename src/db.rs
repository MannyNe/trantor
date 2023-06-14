use std::{convert::Infallible, sync::Arc};

use serde::Serialize;
use sqlx::{
    types::{chrono::NaiveDateTime, BigDecimal},
    FromRow, PgPool,
};
use uaparser::Parser;
use warp::Filter;

use crate::utils;

type Result<T> = std::result::Result<T, sqlx::Error>;

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
        tracing::info!("Extracting tracking id: {:?}", tracking_id);

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
pub struct CountByWeekday {
    #[serde(with = "big_decimal_to_u8")]
    weekday: BigDecimal,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByHour {
    #[serde(with = "big_decimal_to_u8")]
    hour: BigDecimal,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByOs {
    os: String,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByDevice {
    device: String,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByBrowser {
    browser: String,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByPathname {
    pathname: String,
    count: i64,
}

#[derive(FromRow, Serialize)]
pub struct CountByTitle {
    title: String,
    count: i64,
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

    pub async fn count_visitors_by_weekday(&self, tracking_id: i32) -> Result<Vec<CountByWeekday>> {
        let rec = sqlx::query_as!(
            CountByWeekday,
            r#"
            SELECT COUNT(id) as "count!",
                EXTRACT(DOW FROM created_at) as "weekday!"
            FROM visitors
            WHERE tracking_id = $1
            GROUP BY "weekday!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_visitors_by_hour(&self, tracking_id: i32) -> Result<Vec<CountByHour>> {
        let rec = sqlx::query_as!(
            CountByHour,
            r#"
            SELECT COUNT(id) as "count!",
                EXTRACT(HOUR FROM created_at) as "hour!"
            FROM visitors
            WHERE tracking_id = $1
            GROUP BY "hour!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_visitors_by_os(&self, tracking_id: i32) -> Result<Vec<CountByOs>> {
        let rec = sqlx::query_as!(
            CountByOs,
            r#"
            SELECT COUNT(id) as "count!",
                user_agent_parsed->'os'->>'family' AS "os!"
            FROM visitors
            WHERE tracking_id = $1
            GROUP BY "os!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_visitors_by_device(&self, tracking_id: i32) -> Result<Vec<CountByDevice>> {
        let rec = sqlx::query_as!(
            CountByDevice,
            r#"
            SELECT COUNT(id) as "count!",
                user_agent_parsed->'device'->>'family' AS "device!"
            FROM visitors
            WHERE tracking_id = $1
            GROUP BY "device!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_visitors_by_browser(&self, tracking_id: i32) -> Result<Vec<CountByBrowser>> {
        let rec = sqlx::query_as!(
            CountByBrowser,
            r#"
            SELECT COUNT(id) as "count!",
                user_agent_parsed->'user_agent'->>'family' AS "browser!"
            FROM visitors
            WHERE tracking_id = $1
            GROUP BY "browser!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }
}

#[derive(FromRow, Serialize)]
pub struct SingleReferer {
    referer: String,
    visitor_count: i64,
    session_count: i64,
}

impl DB {
    pub async fn list_refers(&self, tracking_id: i32) -> Result<Vec<SingleReferer>> {
        let rec = sqlx::query_as!(
            SingleReferer,
            r#"
            SELECT visitors.referer as referer,
                COUNT(DISTINCT visitors.id) as "visitor_count!",
                COUNT(DISTINCT sessions.id) as "session_count!"
            FROM visitors JOIN sessions ON visitors.id = sessions.visitor_id
            WHERE visitors.tracking_id = $1
            GROUP BY referer
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
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

    pub async fn count_sessions(&self) -> Result<Option<i64>> {
        let rec = sqlx::query!(r#"SELECT COUNT(id) as count FROM sessions"#)
            .fetch_one(&self.pool)
            .await?;

        Ok(rec.count)
    }

    pub async fn count_sessions_by_weekday(&self, tracking_id: i32) -> Result<Vec<CountByWeekday>> {
        let rec = sqlx::query_as!(
            CountByWeekday,
            r#"
            SELECT COUNT(id) as "count!",
                EXTRACT(DOW FROM start_timestamp) as "weekday!"
            FROM sessions
            WHERE tracking_id = $1
            GROUP BY "weekday!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_sessions_by_hour(&self, tracking_id: i32) -> Result<Vec<CountByHour>> {
        let rec = sqlx::query_as!(
            CountByHour,
            r#"
            SELECT COUNT(id) as "count!",
                EXTRACT(HOUR FROM start_timestamp) as "hour!"
            FROM sessions
            WHERE tracking_id = $1
            GROUP BY "hour!"
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_sessions_by_pathname(
        &self,
        tracking_id: i32,
    ) -> Result<Vec<CountByPathname>> {
        let rec = sqlx::query_as!(
            CountByPathname,
            r#"
            SELECT COUNT(DISTINCT sessions.id) as "count!",
                sessions.pathname as pathname
            FROM sessions
            WHERE tracking_id = $1
            GROUP BY pathname
        "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn count_sessions_by_title(&self, tracking_id: i32) -> Result<Vec<CountByTitle>> {
        let rec = sqlx::query_as!(
            CountByTitle,
            r#"
            SELECT COUNT(DISTINCT sessions.id) as "count!",
                sessions.title as title
            FROM sessions
            WHERE tracking_id = $1
            GROUP BY title
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
    session_count: i64,
}

impl DB {
    pub async fn create_source(&self, name: &str, tracking_id: i32) -> Result<()> {
        let _ = sqlx::query!(
            r#"INSERT INTO sources (name, tracking_id) VALUES ($1, $2) RETURNING id"#,
            name,
            tracking_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_source(&self, name: &str, tracking_id: i32) -> Result<()> {
        let _ = sqlx::query!(
            r#"DELETE FROM sources WHERE name = $1 AND tracking_id = $2"#,
            name,
            tracking_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn list_sources(&self, tracking_id: i32) -> Result<Vec<SingleSource>> {
        let sources = sqlx::query_as!(
            SingleSource,
            r#"
            SELECT sources.name as name,
                COUNT(DISTINCT visitors.id) as "visitor_count!",
                COUNT(DISTINCT sessions.id) as "session_count!"
            FROM sources 
                LEFT JOIN visitors ON visitors.source_id = sources.id
                LEFT JOIN sessions ON sessions.visitor_id = visitors.id
            WHERE sources.tracking_id = $1
            GROUP BY sources.name, sources.created_at
            "#,
            tracking_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sources)
    }

    pub async fn visitors_and_sessions_no_source(&self, tracking_id: i32) -> Result<SingleSource> {
        let rec = sqlx::query!(
            r#"
            SELECT COUNT(DISTINCT visitors.id) as "visitor_count!",
                COUNT(DISTINCT sessions.id) as "sessions_count!"
            FROM visitors 
                LEFT JOIN sessions ON sessions.visitor_id = visitors.id
            WHERE visitors.source_id IS NULL AND visitors.tracking_id = $1
            "#,
            tracking_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SingleSource {
            name: "direct".to_owned(),
            visitor_count: rec.visitor_count,
            session_count: rec.sessions_count,
        })
    }

    pub async fn count_sources(&self) -> Result<Option<i64>> {
        let rec = sqlx::query!(r#"SELECT COUNT(id) as count FROM sources"#)
            .fetch_one(&self.pool)
            .await?;

        Ok(rec.count)
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

    pub async fn tracking_owner_and_primary_key(&self, tracking_id: &str) -> Result<(i32, i32)> {
        let rec = sqlx::query!(
            r#"SELECT id, owner_id FROM trackings WHERE tracking_id = $1"#,
            tracking_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok((rec.id, rec.owner_id))
    }

    pub async fn tracking_name(&self, tracking_id: i32) -> Result<String> {
        let rec = sqlx::query!(r#"SELECT name FROM trackings WHERE id = $1"#, tracking_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(rec.name)
    }

    pub async fn delete_tracking(&self, tracking_id: i32) -> Result<()> {
        sqlx::query!(r#"DELETE FROM trackings WHERE id = $1"#, tracking_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn rename_tracking(&self, tracking_id: i32, name: &str) -> Result<()> {
        sqlx::query!(
            r#"UPDATE trackings SET name = $1 WHERE id = $2"#,
            name,
            tracking_id
        )
        .execute(&self.pool)
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

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp_millis())
    }
}

mod big_decimal_to_u8 {
    use num_traits::cast::ToPrimitive;
    use serde::{self, Serializer};
    use sqlx::types::BigDecimal;

    pub fn serialize<S>(decimal: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(decimal.to_u8().unwrap())
    }
}
