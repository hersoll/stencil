use tracing::warn;

pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let default_host = "127.0.0.1";
        let default_port = "3000";

        let host = std::env::var("HOST").unwrap_or_else(|_| {
            warn!("HOST not found in .env, using {default_host}.");
            default_host.to_string()
        });

        let port = std::env::var("PORT").unwrap_or_else(|_| {
            warn!("PORT not found in .env, using {default_port}.");
            default_port.to_string()
        });

        Self { host, port }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
