use async_trait::async_trait;
use thiserror::Error;

use crate::{Location, Session, SessionEnd, UserAgent, Visitor};

#[async_trait]
pub trait Service {
    type Error: std::error::Error;
    type Request;
    type Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}

#[async_trait]
pub trait SessionsRepository {
    async fn create(&self, session: &Session) -> Result<String, SessionRepositoryError>;
    async fn end_session(&self, session_end: &SessionEnd) -> Result<(), SessionRepositoryError>;
}

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("error in sessions repository")]
    Other,
}

#[async_trait]
pub trait VisitorsRepository {
    async fn exists(&self, visitor_id: &str) -> Result<bool, VisitorRepositoryError>;
    async fn create(&self, visitor: &Visitor) -> Result<String, VisitorRepositoryError>;
}

#[derive(Debug, Error)]
pub enum VisitorRepositoryError {
    #[error("error in visitors repository")]
    Other,
}

#[async_trait]
pub trait UserAgentParser {
    async fn parse(&self, user_agent: &str) -> Result<UserAgent, UserAgentParserError>;
}

#[derive(Debug, Error)]
pub enum UserAgentParserError {
    #[error("error in user agent parser")]
    Other,
}

#[async_trait]
pub trait GeoIpReader {
    async fn parse(&self, ip_addr: std::net::IpAddr) -> Result<Location, GeoIpReaderError>;
}

#[derive(Debug, Error)]
pub enum GeoIpReaderError {
    #[error("error in geo ip reader")]
    Other,
}
