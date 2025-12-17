use stencil::server;
use tracing::error;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Set up tracing
    let default_filter = if cfg!(debug_assertions) {
        "debug,stencil=debug,sqlx=warn"
    } else {
        "info,stencil=info,sqlx=warn"
    };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_filter.into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    server::run()
        .await
        .unwrap_or_else(|_| error!("Server run failed!"));
}
