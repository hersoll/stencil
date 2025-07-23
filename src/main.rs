//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]

use dioxus::prelude::*;

#[cfg(feature = "server")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        app::backend::db::init_database()
            .await
            .expect("Failed to initialize db");
    });

    dioxus::launch(app::frontend::AppSetup);
    Ok(())
}
#[cfg(feature = "web")]
fn main() {
    use app::frontend::AppSetup;
    dioxus::launch(AppSetup)
}
