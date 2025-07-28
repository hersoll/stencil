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
    use dioxus::desktop::{
        Config, WindowBuilder,
        wry::dpi::{PhysicalSize, Size},
    };
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Stencil Editor")
                    .with_inner_size(Size::Physical(PhysicalSize {
                        width: 4200,
                        height: 2400,
                    }))
                    .with_resizable(false),
            ),
        )
        .launch(DesktopApp);
}
