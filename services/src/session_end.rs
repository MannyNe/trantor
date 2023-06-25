use domain::{
    async_trait::async_trait, thiserror, Service, SessionEnd, SessionRepositoryError,
    SessionsRepository,
};

#[derive(Clone)]
pub struct SessionEndService<SR> {
    sessions: SR,
}

impl<SR> SessionEndService<SR>
where
    SR: SessionsRepository + Clone + Send,
{
    pub fn new(sessions: SR) -> Self {
        Self { sessions }
    }
}

#[async_trait]
impl<SR> Service for SessionEndService<SR>
where
    SR: SessionsRepository + Sync + Send + Clone,
{
    type Error = SessionEndError;
    type Request = SessionEndRequest;
    type Response = SessionEndResponse;

    async fn execute(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let session_end = SessionEnd::new(req.session_id, req.tracking_id, req.timestamp);
        self.sessions.end_session(&session_end).await?;
        Ok(SessionEndResponse)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SessionEndError {
    #[error("error in sessions repository")]
    SessionsRepository(#[from] SessionRepositoryError),
}

pub struct SessionEndRequest {
    tracking_id: String,
    session_id: String,
    timestamp: f64,
}

impl SessionEndRequest {
    pub fn new(tracking_id: String, session_id: String, timestamp: f64) -> Self {
        Self {
            tracking_id,
            session_id,
            timestamp,
        }
    }
}

pub struct SessionEndResponse;
