use async_trait::async_trait;
use thiserror::Error;

use super::models::{LocationData, SessionModel, UserAgentData, VisitorModel};

#[async_trait]
pub trait SessionsRepository {
    async fn create(&self, session: &SessionModel) -> Result<String, SessionRepositoryError>;
}

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("error in sessions repository")]
    Other,
}

#[async_trait]
pub trait VisitorsRepository {
    async fn exists(&self, visitor_id: &str) -> Result<bool, VisitorRepositoryError>;
    async fn create(&self, visitor: &VisitorModel) -> Result<String, VisitorRepositoryError>;
}

#[derive(Debug, Error)]
pub enum VisitorRepositoryError {
    #[error("error in visitors repository")]
    Other,
}

#[async_trait]
pub trait UserAgentParserPort {
    async fn parse(&self, user_agent: &str) -> Result<UserAgentData, UserAgentParserError>;
}

#[derive(Debug, Error)]
pub enum UserAgentParserError {
    #[error("error in user agent parser")]
    Other,
}

#[async_trait]
pub trait GeoIpReader {
    async fn parse(&self, ip_addr: std::net::IpAddr) -> Result<LocationData, GeoIpReaderError>;
}

#[derive(Debug, Error)]
pub enum GeoIpReaderError {
    #[error("error in geo ip reader")]
    Other,
}

#[async_trait]
pub trait UseCase {
    type Error: std::fmt::Debug;
    type Request;
    type Response;

    async fn execute(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;
}
