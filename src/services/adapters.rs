use std::sync::Arc;

use async_trait::async_trait;

use super::{
    models::{LocationData, SessionModel, UserAgentData, VisitorModel},
    ports::{
        GeoIpReader, GeoIpReaderError, SessionRepositoryError, SessionsRepository,
        UserAgentParserError, UserAgentParserPort, VisitorRepositoryError, VisitorsRepository,
    },
};

#[derive(Clone)]
pub struct PgSessionsRepository {
    pool: sqlx::PgPool,
}

impl PgSessionsRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create(&self, session: &SessionModel) -> Result<String, SessionRepositoryError> {
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
        .await?;

        Ok(record.session_id)
    }
}

impl From<sqlx::Error> for SessionRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("error in sessions repository: {}", err);
        Self::Other
    }
}

#[derive(Clone)]
pub struct PgVisitorsRepository {
    pool: sqlx::PgPool,
}

impl PgVisitorsRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
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
        .await?;

        Ok(record.is_some())
    }

    async fn create(&self, visitor: &VisitorModel) -> Result<String, VisitorRepositoryError> {
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
        .await?;

        Ok(record.visitor_id)
    }
}

impl From<sqlx::Error> for VisitorRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("error in visitors repository: {}", err);
        Self::Other
    }
}

#[derive(Clone)]
pub struct UAParser {
    parser: Arc<uaparser::UserAgentParser>,
}

impl UAParser {
    pub fn new(parser: uaparser::UserAgentParser) -> Self {
        Self {
            parser: Arc::new(parser),
        }
    }
}

#[async_trait]
impl UserAgentParserPort for UAParser {
    async fn parse(&self, user_agent: &str) -> Result<UserAgentData, UserAgentParserError> {
        use uaparser::Parser;
        let user_agent = self.parser.parse(user_agent);

        let device = user_agent.device.family;
        let os = user_agent.os.family;
        let user_agent = user_agent.user_agent.family;

        Ok(UserAgentData::new(
            device.to_string(),
            os.to_string(),
            user_agent.to_string(),
        ))
    }
}

#[derive(Clone)]
pub struct MaxmindGeoIpReader {
    reader: Arc<maxminddb::Reader<Vec<u8>>>,
}

impl MaxmindGeoIpReader {
    pub fn new(reader: maxminddb::Reader<Vec<u8>>) -> Self {
        Self {
            reader: Arc::new(reader),
        }
    }
}

#[async_trait]
impl GeoIpReader for MaxmindGeoIpReader {
    async fn parse(&self, ip_addr: std::net::IpAddr) -> Result<LocationData, GeoIpReaderError> {
        let location = self.reader.lookup::<maxminddb::geoip2::City>(ip_addr)?;

        let country_code = location
            .country
            .and_then(|c| c.iso_code)
            .map(|iso| iso.to_owned());
        let city_name = location
            .city
            .and_then(|c| c.names)
            .and_then(|ns| ns.get("en").copied())
            .map(|n| n.to_owned());
        let continent_code = location
            .continent
            .and_then(|c| c.code)
            .map(|c| c.to_owned());

        Ok(LocationData::new(country_code, city_name, continent_code))
    }
}

impl From<maxminddb::MaxMindDBError> for GeoIpReaderError {
    fn from(err: maxminddb::MaxMindDBError) -> Self {
        tracing::error!("error in geoip reader: {}", err);
        Self::Other
    }
}
