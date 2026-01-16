use axum::{
    Extension, Json, Router,
    extract::{Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
};

use serde::Serialize;

use super::middleware::auth_middleware;
use super::rbac::{Resource, Scope, Verb};
use crate::extractors::QueryParams;
use crate::{api::AppState, extractors::ValidatedPath};
use common::{Policy, Stats, TagStats};

#[derive(Debug, Serialize)]
struct Metrics {
    #[serde(flatten)]
    stats: Stats,
    drop_tags: TagStats,
    ignore_tags: TagStats,
}

/// Helper function to convert all stats to Prometheus format
fn prometheus_report(stats: &Stats, drop_tags: &TagStats, ignore_tags: &TagStats) -> String {
    let mut report = String::with_capacity(4096);

    // couic_drop_cidr_total
    report.push_str("# HELP couic_drop_cidr_total Current number of CIDR dropped by couic.\n");
    report.push_str("# TYPE couic_drop_cidr_total gauge\n");
    report.push_str(&format!(
        "couic_drop_cidr_total {}\n",
        stats.drop_cidr_count
    ));

    // couic_ignore_cidr_total
    report.push_str("# HELP couic_ignore_cidr_total Current number of CIDR ignored by couic.\n");
    report.push_str("# TYPE couic_ignore_cidr_total gauge\n");
    report.push_str(&format!(
        "couic_ignore_cidr_total {}\n",
        stats.ignore_cidr_count
    ));

    // couic_stats_rx_packets_total
    report.push_str(
        "# HELP couic_stats_rx_packets_total Current number of packets handled by XDP.\n",
    );
    report.push_str("# TYPE couic_stats_rx_packets_total counter\n");
    for (k, v) in &stats.xdp {
        report.push_str(&format!(
            "couic_stats_rx_packets_total{{action=\"{}\"}} {}\n",
            k, v.rx_packets
        ));
    }

    // couic_stats_rx_bytes_total
    report.push_str("# HELP couic_stats_rx_bytes_total Current number of bytes handled by XDP.\n");
    report.push_str("# TYPE couic_stats_rx_bytes_total counter\n");
    for (k, v) in &stats.xdp {
        report.push_str(&format!(
            "couic_stats_rx_bytes_total{{action=\"{}\"}} {}\n",
            k, v.rx_bytes
        ));
    }

    // couic_drop_tag_rx_packets_total
    report.push_str("# HELP couic_drop_tag_rx_packets_total Number of packets dropped per tag.\n");
    report.push_str("# TYPE couic_drop_tag_rx_packets_total counter\n");
    for (tag, pkt_stats) in &drop_tags.tags {
        report.push_str(&format!(
            "couic_drop_tag_rx_packets_total{{tag=\"{}\"}} {}\n",
            tag, pkt_stats.rx_packets
        ));
    }

    // couic_drop_tag_rx_bytes_total
    report.push_str("# HELP couic_drop_tag_rx_bytes_total Number of bytes dropped per tag.\n");
    report.push_str("# TYPE couic_drop_tag_rx_bytes_total counter\n");
    for (tag, pkt_stats) in &drop_tags.tags {
        report.push_str(&format!(
            "couic_drop_tag_rx_bytes_total{{tag=\"{}\"}} {}\n",
            tag, pkt_stats.rx_bytes
        ));
    }

    // couic_ignore_tag_rx_packets_total
    report
        .push_str("# HELP couic_ignore_tag_rx_packets_total Number of packets ignored per tag.\n");
    report.push_str("# TYPE couic_ignore_tag_rx_packets_total counter\n");
    for (tag, pkt_stats) in &ignore_tags.tags {
        report.push_str(&format!(
            "couic_ignore_tag_rx_packets_total{{tag=\"{}\"}} {}\n",
            tag, pkt_stats.rx_packets
        ));
    }

    // couic_ignore_tag_rx_bytes_total
    report.push_str("# HELP couic_ignore_tag_rx_bytes_total Number of bytes ignored per tag.\n");
    report.push_str("# TYPE couic_ignore_tag_rx_bytes_total counter\n");
    for (tag, pkt_stats) in &ignore_tags.tags {
        report.push_str(&format!(
            "couic_ignore_tag_rx_bytes_total{{tag=\"{}\"}} {}\n",
            tag, pkt_stats.rx_bytes
        ));
    }

    // OpenMetrics requires EOF marker
    report.push_str("# EOF\n");

    report
}

/// Handler for XDP statistics endpoint
async fn get_stats(State(state): State<AppState>) -> impl IntoResponse {
    match state.firewall_service.get_stats() {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(ce) => ce.into_response(),
    }
}

/// Handler for statistics per tag endpoint
async fn get_stats_tag(
    State(state): State<AppState>,
    ValidatedPath(policy): ValidatedPath<Policy>,
) -> impl IntoResponse {
    match state.firewall_service.get_stats_tags(policy) {
        Ok(tag_stats) => (StatusCode::OK, Json(tag_stats)).into_response(),
        Err(ce) => ce.into_response(),
    }
}

/// Handler for metrics endpoint
async fn get_metrics(
    State(state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> impl IntoResponse {
    let stats = match state.firewall_service.get_stats() {
        Ok(s) => s,
        Err(ce) => return ce.into_response(),
    };

    let drop_tags = match state.firewall_service.get_stats_tags(Policy::Drop) {
        Ok(t) => t,
        Err(ce) => return ce.into_response(),
    };

    let ignore_tags = match state.firewall_service.get_stats_tags(Policy::Ignore) {
        Ok(t) => t,
        Err(ce) => return ce.into_response(),
    };

    if params.format.as_deref() == Some("prometheus") {
        let metrics_text = prometheus_report(&stats, &drop_tags, &ignore_tags);
        (
            StatusCode::OK,
            [(
                "content-type",
                "application/openmetrics-text; version=1.0.0; charset=utf-8",
            )],
            metrics_text,
        )
            .into_response()
    } else {
        let metrics = Metrics {
            stats,
            drop_tags,
            ignore_tags,
        };
        (StatusCode::OK, Json(metrics)).into_response()
    }
}

/// Create router for stats endpoints
pub(super) fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/v1/stats",
            get(get_stats)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Stats, Verb::List))),
        )
        .route(
            "/v1/stats/tags/{policy}",
            get(get_stats_tag)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Stats, Verb::Get))),
        )
        .route(
            "/v1/metrics",
            get(get_metrics)
                .route_layer(middleware::from_fn_with_state(state, auth_middleware))
                .route_layer(Extension(Scope::with(Resource::Stats, Verb::List))),
        )
}
