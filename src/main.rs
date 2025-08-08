use tracing_subscriber;

#[cfg(feature = "server")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        app::backend::db::init_database()
            .await
            .expect("Failed to initialize db");

        println!("Loading problems to registry...");
        app::backend::load_problem_data().await;
        println!("Problems loaded!");
        println!("Loading prefixes to registry...");
        app::backend::load_prefix_data().await;
        println!("Prefixes loaded!");
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
    use app::editor::EditorApp;
    use dioxus::desktop::{
        Config, WindowBuilder,
        wry::dpi::{PhysicalSize, Size},
    };
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Stencil Editor")
                    .with_focused(true)
                    .with_inner_size(Size::Physical(PhysicalSize {
                        width: 2800,
                        height: 2000,
                    }))
                    .with_resizable(false),
            ),
        )
        .launch(EditorApp);
}
