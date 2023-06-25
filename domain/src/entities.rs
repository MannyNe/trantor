pub struct Session {
    session_id: String,
    tracking_id: String,
    visitor_id: String,
    timestamp: f64,
    title: String,
    pathname: String,
    referral: Option<String>,
    location: Location,
}

impl Session {
    pub fn new(
        tracking_id: &str,
        visitor_id: String,
        timestamp: f64,
        title: String,
        pathname: String,
        referral: Option<String>,
        location: Location,
    ) -> Self {
        Self {
            session_id: crate::utils::generate_id(),
            tracking_id: tracking_id.to_owned(),
            visitor_id,
            timestamp,
            title,
            pathname,
            referral,
            location,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn tracking_id(&self) -> &str {
        &self.tracking_id
    }

    pub fn visitor_id(&self) -> &str {
        &self.visitor_id
    }

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn pathname(&self) -> &str {
        &self.pathname
    }

    pub fn referral(&self) -> Option<&str> {
        self.referral.as_deref()
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

pub struct SessionEnd {
    session_id: String,
    tracking_id: String,
    timestamp: f64,
}

impl SessionEnd {
    pub fn new(session_id: String, tracking_id: String, timestamp: f64) -> Self {
        Self {
            session_id,
            tracking_id,
            timestamp,
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn tracking_id(&self) -> &str {
        &self.tracking_id
    }

    pub fn timestamp(&self) -> f64 {
        self.timestamp
    }
}

pub struct Location {
    country_code: Option<String>,
    city_name: Option<String>,
    continent_code: Option<String>,
}

impl Location {
    pub fn new(
        country_code: Option<String>,
        city_name: Option<String>,
        continent_code: Option<String>,
    ) -> Self {
        Self {
            country_code,
            city_name,
            continent_code,
        }
    }

    pub fn country_code(&self) -> Option<&str> {
        self.country_code.as_deref()
    }

    pub fn city_name(&self) -> Option<&str> {
        self.city_name.as_deref()
    }

    pub fn continent_code(&self) -> Option<&str> {
        self.continent_code.as_deref()
    }
}

pub struct Visitor {
    visitor_id: String,
    tracking_id: String,
    source_name: Option<String>,
    referer: String,
    user_agent: UserAgent,
}

impl Visitor {
    pub fn new(
        tracking_id: &str,
        source_name: Option<String>,
        referer: String,
        user_agent: UserAgent,
    ) -> Self {
        Self {
            visitor_id: crate::utils::generate_id(),
            tracking_id: tracking_id.to_owned(),
            source_name,
            referer,
            user_agent,
        }
    }

    pub fn visitor_id(&self) -> &str {
        &self.visitor_id
    }

    pub fn tracking_id(&self) -> &str {
        &self.tracking_id
    }

    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    pub fn referer(&self) -> &str {
        &self.referer
    }

    pub fn user_agent(&self) -> &UserAgent {
        &self.user_agent
    }
}

pub struct UserAgent {
    device: String,
    os: String,
    user_agent: String,
}

impl UserAgent {
    pub fn new(device: String, os: String, user_agent: String) -> Self {
        Self {
            device,
            os,
            user_agent,
        }
    }

    pub fn device(&self) -> &str {
        &self.device
    }

    pub fn os(&self) -> &str {
        &self.os
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
}
