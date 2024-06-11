use std::env::var;

use log::LevelFilter;

#[derive(Debug, Clone)]
pub struct DBConfig {
    uri: Box<str>,
    name: Box<str>,
    host: Option<Box<str>>,
    user: Option<Box<str>>,
    port: Option<u16>,
    password: Option<Box<str>>,
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    connect_timeout: Option<u64>,
    acquire_timeout: Option<u64>,
    idle_timeout: Option<u64>,
    max_lifetime: Option<u64>,
    sqlx_logging: Option<bool>,
    sqlx_logging_level: Option<usize>

}

impl DBConfig {

    fn new() -> Self {
        DBConfig {
            uri: var("DB_URI").expect("DB_URI must be set").into_boxed_str(),
            name: var("DB_NAME").expect("DB_NAME must be set").into_boxed_str(),
            host: var("DB_HOST").ok().map(|h| h.into_boxed_str()),
            user: var("DB_USER").ok().map(|u| u.into_boxed_str()),
            password: var("DB_PASSWORD").ok().map(|p| p.into_boxed_str()),
            port: var("DB_PORT").ok().and_then(|port| port.parse().ok()),
            max_connections: var("DB_MAX_CONNECTIONS").ok().and_then(|max| max.parse().ok()).or(Some(100)),
            min_connections: var("DB_MIN_CONNECTIONS").ok().and_then(|min| min.parse().ok()).or(Some(5)),
            connect_timeout: var("DB_CONNECTION_TIMEOUT").ok().and_then(|t| t.parse().ok()).or(Some(30)),
            acquire_timeout: var("DB_ACQUIRE_TIMEOUT").ok().and_then(|t| t.parse().ok()).or(Some(30)),
            idle_timeout: var("DB_IDLE_TIMEOUT").ok().and_then(|t| t.parse().ok()).or(Some(600)),
            max_lifetime: var("DB_MAX_LIFETIME").ok().and_then(|t| t.parse().ok()).or(Some(1800)),
            sqlx_logging: var("DB_SQLX_LOGGING").ok().and_then(|l| l.parse().ok()).or(Some(true)),
            sqlx_logging_level: var("DB_SQLX_LOGGING_LEVEL").ok().and_then(|ll| ll.parse().ok()).or(Some(3))
        }
    }

    pub fn url(&self) -> String {

        if self.uri.contains("sqlite") {
            self.uri.replace("{name}", &self.name)
        } else {
            self.uri
                .replace("{user}", self.user.as_ref().expect("DB_USER must be set"))
                .replace("{password}", self.password.as_ref().expect("DB_PASSWORD must be set"))
                .replace("{host}", self.host.as_ref().expect("DB_HOST must be set"))
                .replace("{port}", &self.port.expect("DB_PORT must be set").to_string())
                .replace("{name}", &self.name)
        }
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections.expect("max_connections was not set")
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections.expect("min_connections was not set")
    }

    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout.expect("connect_timeout was not set")
    }

    pub fn acquire_timeout(&self) -> u64 {
        self.acquire_timeout.expect("acquire_timeout was not set")
    }

    pub fn idle_timeout(&self) -> u64 {
        self.idle_timeout.expect("idle_timeout was not set")
    }

    pub fn max_lifetime(&self) -> u64 {
        self.max_lifetime.expect("max_lifetime was not set")
    }

    pub fn sqlx_logging(&self) -> bool {
        self.sqlx_logging.expect("sqlx_logging was not set")
    }

    pub fn sqlx_logging_level(&self) -> LevelFilter {
        match self.sqlx_logging_level {
            Some(0) => LevelFilter::Off,
            Some(1) => LevelFilter::Error,
            Some(2) => LevelFilter::Warn,
            Some(3) => LevelFilter::Info,
            Some(4) => LevelFilter::Debug,
            Some(5) => LevelFilter::Trace,
            Some(_) => LevelFilter::Info,
            None => LevelFilter::Info
        }
    }

}

#[derive(Debug, Clone)]
pub struct TokenConfig {
    pub algorithm: Box<str>, 
    pub secret_key: Box<str>, 
    pub public_key: Option<Box<str>>,  
    pub access_token_expire_seconds: i64, 
    pub refresh_token_expire_seconds: i64
}

impl TokenConfig {
    fn new() -> Self {
        let secret_key = var("SECRET_KEY").expect("SECRET_KEY must be set").into_boxed_str();
        Self {
            algorithm: var("ALGORITHM").expect("ALGORITHM must be set").into_boxed_str(),
            secret_key: secret_key.clone(),
            public_key: var("PUBLIC_KEY").ok().map(|key| key.into_boxed_str()).or(Some(secret_key)),
            access_token_expire_seconds: var("ACCESS_TOKEN_EXPIRE_SECONDS")
                .expect("ACCESS_TOKEN_EXPIRE_SECONDS must be set")
                .parse::<i64>()
                .expect("ACCESS_TOKEN_EXPIRE_SECONDS must be an integer type"),
            refresh_token_expire_seconds: var("REFRESH_TOKEN_EXPIRE_SECONDS")
                .expect("REFRESH_TOKEN_EXPIRE_SECONDS must be set")
                .parse::<i64>()
                .expect("REFRESH_TOKEN_EXPIRE_SECONDS must be an integer type"),
        }
    }
}



#[derive(Debug, Clone)]
pub struct ServerConfig {
    host: Option<Box<str>>,
    port: Option<u16>
}

impl ServerConfig {
    fn new() -> Self {
        ServerConfig {
            host: var("SERVER_HOST").ok().map(|h| h.into_boxed_str()).or(Some("0.0.0.0".into())),
            port: var("SERVER_PORT").ok().and_then(|p| p.parse().ok().or(Some(8080)))
        }
    }
    
    pub fn host(&self) -> &str {
        self.host.as_ref().expect("host was not set")
    }

    pub fn port(&self) -> u16 {
        self.port.expect("port was not set")
    }

}

#[derive(Debug, Clone)]
pub struct Config {
    pub db: DBConfig,
    pub server: ServerConfig,
    pub token: TokenConfig,
}

impl Config {
    pub fn new() -> Self {
        Config {
            db: DBConfig::new(),
            server: ServerConfig::new(),
            token: TokenConfig::new()
        }
    }
}