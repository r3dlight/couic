mod clients;
mod middleware;
mod policies;
pub mod rbac;
mod sets;
mod stats;

use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;

use crate::firewall::service::FirewallService;
use rbac::RBACService;

#[derive(Clone)]
pub(crate) struct AppState {
    firewall_service: Arc<FirewallService>,
    rbac_service: Arc<RwLock<RBACService>>,
}

impl AppState {
    pub fn new(firewall_service: FirewallService, rbac_service: RBACService) -> Self {
        Self {
            firewall_service: Arc::new(firewall_service),
            rbac_service: Arc::new(RwLock::new(rbac_service)),
        }
    }
}

pub fn create_router(firewall_service: FirewallService, rbac_service: RBACService) -> Router {
    let state = AppState::new(firewall_service, rbac_service);

    Router::new()
        .merge(policies::router(state.clone()))
        .merge(sets::router(state.clone()))
        .merge(stats::router(state.clone()))
        .merge(clients::router(state.clone()))
        .with_state(state)
}
