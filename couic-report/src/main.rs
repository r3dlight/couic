mod config;
mod notifier;
mod stats;
mod worker;

use axum::{Json, Router, extract::Path, extract::State, http::StatusCode, routing::post};
use clap::Parser;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc};
use tracing::{debug, error, info};
use uuid::Uuid;

use common::Action;

use crate::{
    config::load_config,
    notifier::{NotificationDispatcher, Notifier, discord::DiscordNotifier},
    stats::Report,
    worker::start_worker,
};

#[derive(Clone)]
struct AppState {
    tx: mpsc::UnboundedSender<Vec<Report>>,
    secret: Uuid,
}

async fn report_handler(
    Path(secret): Path<String>,
    State(state): State<AppState>,
    Json(reports): Json<Vec<Report>>,
) -> Result<&'static str, StatusCode> {
    if secret != state.secret.to_string() {
        debug!("Invalid secret received");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Filter only reports with action == Add
    let filtered: Vec<Report> = reports
        .into_iter()
        .filter(|r| r.action == Action::Add)
        .collect();
    if !filtered.is_empty() {
        debug!("Received {} valid reports", filtered.len());
        state.tx.send(filtered).expect("send to worker");
    } else {
        debug!("No valid reports received");
    }
    Ok("reports received")
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(
        short = 'c',
        long = "config",
        default_value = "/etc/couic-report/config.toml"
    )]
    config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let config_path = args.config;

    let cfg = match load_config(&config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Configuration error: {e}");
            std::process::exit(1);
        }
    };

    let mut notifiers: Vec<Arc<dyn Notifier>> = Vec::new();

    if let Some(dc) = cfg.discord {
        info!("Discord notifier enabled");
        notifiers.push(Arc::new(DiscordNotifier::new(
            dc.webhook_url,
            cfg.server.name.clone(),
            cfg.batch_interval_secs,
            cfg.thresholds.clone(),
        )));
    }

    let dispatcher = Arc::new(NotificationDispatcher::new(notifiers));

    let (tx, rx) = mpsc::unbounded_channel::<Vec<Report>>();
    tokio::spawn(start_worker(
        rx,
        dispatcher,
        cfg.batch_interval_secs,
        cfg.thresholds.clone(),
    ));

    let app_state = AppState {
        tx,
        secret: cfg.server.secret,
    };

    let app = Router::new()
        .route("/v1/reports/{secret}", post(report_handler))
        .with_state(app_state);

    let bind_addr = format!("{}:{}", cfg.server.addr, cfg.server.port);
    let listener = TcpListener::bind(&bind_addr)
        .await
        .expect("Failed to bind TCP listener");

    info!("Server running on {}", bind_addr);
    axum::serve(listener, app).await.unwrap();
}
