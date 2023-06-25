use domain::{
    async_trait::async_trait, tracing, Session, SessionEnd, SessionRepositoryError,
    SessionsRepository, Visitor, VisitorRepositoryError, VisitorsRepository,
};

pub use sqlx;

struct SqlxError(sqlx::Error);

#[derive(Clone)]
pub struct PgSessionsRepository {
    pool: sqlx::PgPool,
}

impl PgSessionsRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create(&self, session: &Session) -> Result<String, SessionRepositoryError> {
        let record = sqlx::query!(
            r#"
insert into sessions (
    session_id,
    tracking_id,
    visitor_id,
    start_timestamp,
    title,
    pathname,
    referral,
    country_code,
    city_name,
    continent_code
  )
values (
    $1,
    (
      select id
      from trackings
      where tracking_id = $2
    ),
    (
      select id
      from visitors
      where visitor_id = $3
    ),
    TO_TIMESTAMP($4),
    $5,
    $6,
    $7,
    $8,
    $9,
    $10
  ) returning session_id
"#,
            session.session_id(),
            session.tracking_id(),
            session.visitor_id(),
            session.timestamp(),
            session.title(),
            session.pathname(),
            session.referral(),
            session.location().country_code(),
            session.location().city_name(),
            session.location().continent_code(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(SqlxError)?;

        Ok(record.session_id)
    }

    async fn end_session(&self, session_end: &SessionEnd) -> Result<(), SessionRepositoryError> {
        sqlx::query!(
            r#"
update sessions
set ended_at = CURRENT_TIMESTAMP,
  end_timestamp = TO_TIMESTAMP($1)
where tracking_id = (
    select id
    from trackings
    where tracking_id = $2
  )
  and session_id = $3
"#,
            session_end.timestamp(),
            session_end.tracking_id(),
            session_end.session_id()
        )
        .execute(&self.pool)
        .await
        .map_err(SqlxError)?;

        Ok(())
    }
}

impl From<SqlxError> for SessionRepositoryError {
    fn from(err: SqlxError) -> Self {
        tracing::error!("error in sessions repository: {}", err.0);
        Self::Other
    }
}

#[derive(Clone)]
pub struct PgVisitorsRepository {
    pool: sqlx::PgPool,
}

impl PgVisitorsRepository {
    pub fn new(pool: &sqlx::PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl VisitorsRepository for PgVisitorsRepository {
    async fn exists(&self, visitor_id: &str) -> Result<bool, VisitorRepositoryError> {
        let record = sqlx::query!(
            r#"
select id
from visitors
where visitor_id = $1
"#,
            visitor_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(SqlxError)?;

        Ok(record.is_some())
    }

    async fn create(&self, visitor: &Visitor) -> Result<String, VisitorRepositoryError> {
        let record = sqlx::query!(
            r#"
insert into visitors (
    visitor_id,
    tracking_id,
    source_id,
    referer,
    user_agent,
    user_agent_device,
    user_agent_os
  )
values (
    $1,
    (
      select id
      from trackings
      where tracking_id = $2
    ),
    (
      select id
      from sources
      where name = $3
    ),
    $4,
    $5,
    $6,
    $7
  ) returning visitor_id;
"#,
            visitor.visitor_id(),
            visitor.tracking_id(),
            visitor.source_name(),
            visitor.referer(),
            visitor.user_agent().user_agent(),
            visitor.user_agent().device(),
            visitor.user_agent().os(),
        )
        .fetch_one(&self.pool)
        .await
        .map_err(SqlxError)?;

        Ok(record.visitor_id)
    }
}

impl From<SqlxError> for VisitorRepositoryError {
    fn from(err: SqlxError) -> Self {
        tracing::error!("error in visitors repository: {}", err.0);
        Self::Other
    }
}
