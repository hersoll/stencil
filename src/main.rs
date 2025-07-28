use tracing_subscriber;

#[cfg(feature = "server")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        app::backend::db::init_database()
            .await
            .expect("Failed to initialize db");
    });

    tracing_subscriber::fmt::init();

    dioxus::launch(app::frontend::AppSetup);
    Ok(())
}

#[cfg(feature = "web")]
fn main() {
    use app::frontend::AppSetup;
    dioxus::launch(AppSetup)
}

#[cfg(feature = "desktop")]
fn main() {
    use app::desktop::DesktopApp;
    dioxus::launch(DesktopApp);
}
