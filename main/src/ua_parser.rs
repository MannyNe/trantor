use domain::{async_trait::async_trait, UserAgent, UserAgentParser, UserAgentParserError};
use std::sync::Arc;

const REGEXES: &[u8; 205550] = include_bytes!("ua-regexes.yml");

#[derive(Clone)]
pub struct UAParser {
    parser: Arc<uaparser::UserAgentParser>,
}

impl UAParser {
    pub fn new() -> Self {
        let parser = uaparser::UserAgentParser::from_bytes(REGEXES)
            .expect("failed to create user agent parser");
        Self {
            parser: Arc::new(parser),
        }
    }
}

#[async_trait]
impl UserAgentParser for UAParser {
    async fn parse(&self, user_agent: &str) -> Result<UserAgent, UserAgentParserError> {
        use uaparser::Parser;
        let user_agent = self.parser.parse(user_agent);

        let device = user_agent.device.family;
        let os = user_agent.os.family;
        let user_agent = user_agent.user_agent.family;

        Ok(UserAgent::new(
            device.to_string(),
            os.to_string(),
            user_agent.to_string(),
        ))
    }
}
