use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tokio::sync::RwLock;
use monitor::{Monitor, Metrics};
use std::sync::Arc;

mod monitor;

async fn get_metrics(monitor: web::Data<Arc<Monitor>>) -> impl Responder {
    let metrics = monitor.gather_metrics().await;
    HttpResponse::Ok().json(metrics)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let monitor = Arc::new(Monitor::new());

    // Start monitoring in background
    let monitor_clone = Arc::clone(&monitor);
    tokio::spawn(async move {
        monitor_clone.start_monitoring().await;
    });

    // Run web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&monitor)))
            .route("/metrics", web::get().to(get_metrics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
