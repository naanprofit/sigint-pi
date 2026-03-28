mod api;

use actix_web::{web, App, HttpServer, middleware};
use std::sync::Arc;
use tokio::sync::broadcast;
use anyhow::Result;

use crate::config::Config;
use crate::storage::Database;
use crate::ScanEvent;

pub async fn start_server(
    db: Arc<Database>,
    config: Arc<Config>,
    _event_rx: broadcast::Receiver<ScanEvent>,
) -> Result<()> {
    let db_data = web::Data::new(db);
    let config_data = web::Data::new(config.clone());
    
    let bind_addr = format!("{}:{}", config.web.bind_address, config.web.port);
    
    // Run actix in its own system
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .app_data(config_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .configure(api::configure)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run();
    
    server.await?;

    Ok(())
}
