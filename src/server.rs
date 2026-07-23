use {
    crate::{
        layouts::{self, header},
        middlewares,
        pages,
        shared::wini::{layer::MetaLayerBuilder, PORT},
        template,
        utils::wini::{
            cache,
            handling_file::{self},
        },
    },
    axum::{middleware, routing::get, Router},
    log::info,
    std::collections::HashMap,
    tower_http::compression::CompressionLayer,
};


pub async fn start() {
    // let api_router = Router::new().route("/lua/page", method_router);

    let page_router = Router::new()
        .route("/lua", get(pages::lua::render))
        .route("/lua/{*wildcard}", get(pages::lua::render))
        .layer(middleware::from_fn(layouts::lua::render))
        // The editor has its own full-screen layout, so it is registered after
        // the `lua` layout middleware to bypass the tutorial sidebar.
        .route("/lua/editor", get(pages::lua::editor::render))
        .route("/", get(pages::home::render));

    let datastar_router = Router::new()
        .layer(middleware::from_fn(middlewares::datastar::replace))
        .route("/lua", get(pages::lua::datastar))
        .route("/lua/{*wildcard}", get(pages::lua::datastar))
        .layer(middleware::from_fn(middlewares::datastar::compress_sse))
        .layer(CompressionLayer::new());

    let app = page_router
        .layer(
            MetaLayerBuilder::default()
                .default_meta(HashMap::from_iter([
                    ("title", "tk's tutorial".into()),
                    ("description", "Websites of tutorials by tk".into()),
                ]))
                .build()
                .expect("Failed to build MetaLayer"),
        )
        .layer(middleware::from_fn(template::template))
        .layer(middleware::from_fn(cache::html_middleware))
        .nest("/ds", datastar_router)
        .route("/{*wildcard}", get(handling_file::handle_file))
        .layer(CompressionLayer::new());


    // Start the server
    info!("Starting listening on port {}...", *PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", *PORT))
        .await
        .expect("Couldn't start the TcpListener of the specified port.");

    info!("Starting the server...");
    axum::serve(listener, app)
        .await
        .expect("Couldn't start the server.");
}
