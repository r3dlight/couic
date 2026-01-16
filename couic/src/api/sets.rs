use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use tracing::{error, info};

use super::middleware::auth_middleware;
use super::rbac::{Resource, Scope, Verb};
use crate::extractors::ValidatedJson;
use crate::{api::AppState, extractors::ValidatedPath};
use common::{Client, Policy, Set, SetPath};

/// List all sets for a given policy
async fn list_sets(
    State(state): State<AppState>,
    ValidatedPath(policy): ValidatedPath<Policy>,
) -> impl IntoResponse {
    match state.firewall_service.list_sets(policy) {
        Ok(sets) => (StatusCode::OK, Json(sets)).into_response(),
        Err(ce) => ce.into_response(),
    }
}

/// Create a new set
async fn create_set(
    State(state): State<AppState>,
    ValidatedPath(policy): ValidatedPath<Policy>,
    Extension(client): Extension<Client>,
    ValidatedJson(set): ValidatedJson<Set>,
) -> impl IntoResponse {
    match state
        .firewall_service
        .create_set(policy, &set.name, &set.entries)
    {
        Ok(response) => {
            info!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %set.name,
                set.entry_count = set.entries.len(),
                "set created"
            );
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(ce) => {
            error!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %set.name,
                error = %ce,
                "failed to create set"
            );
            ce.into_response()
        }
    }
}

/// Get a specific set
async fn get_set(
    State(state): State<AppState>,
    ValidatedPath(SetPath { policy, name }): ValidatedPath<SetPath>,
) -> impl IntoResponse {
    match state.firewall_service.get_set(policy, &name) {
        Ok(set) => (StatusCode::OK, Json(set)).into_response(),
        Err(ce) => ce.into_response(),
    }
}

/// Update a set (replace all entries)
async fn update_set(
    State(state): State<AppState>,
    ValidatedPath(SetPath { policy, name }): ValidatedPath<SetPath>,
    Extension(client): Extension<Client>,
    ValidatedJson(set): ValidatedJson<Set>,
) -> impl IntoResponse {
    match state
        .firewall_service
        .update_set(policy, &name, &set.entries)
    {
        Ok(response) => {
            info!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %name,
                set.entry_count = set.entries.len(),
                "set updated"
            );
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(ce) => {
            error!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %name,
                error = %ce,
                "failed to update set"
            );
            ce.into_response()
        }
    }
}

/// Delete a set
async fn delete_set(
    State(state): State<AppState>,
    ValidatedPath(SetPath { policy, name }): ValidatedPath<SetPath>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    match state.firewall_service.delete_set(policy, &name) {
        Ok(()) => {
            info!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %name,
                "set deleted"
            );
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "deleted": true,
                    "reload_required": true
                })),
            )
                .into_response()
        }
        Err(ce) => {
            error!(
                client.name = %client.name,
                client.group = %client.group,
                policy = %policy,
                set.name = %name,
                error = %ce,
                "failed to delete set"
            );
            ce.into_response()
        }
    }
}

/// Handler for reloading sets
async fn post_sets_reload(
    State(state): State<AppState>,
    Extension(client): Extension<Client>,
) -> impl IntoResponse {
    match state.firewall_service.reload_sets() {
        Ok(()) => {
            info!(
                client.name = %client.name,
                client.group = %client.group,
                "sets reloaded"
            );
            (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "reload_status": "OK"
                })),
            )
                .into_response()
        }
        Err(ce) => {
            error!(
                client.name = %client.name,
                client.group = %client.group,
                error = %ce,
                "failed to reload sets"
            );
            ce.into_response()
        }
    }
}

/// Create router for sets endpoints
pub(super) fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/v1/sets/{policy}",
            get(list_sets)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::List))),
        )
        .route(
            "/v1/sets/{policy}",
            post(create_set)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::Create))),
        )
        .route(
            "/v1/sets/{policy}/{name}",
            get(get_set)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::Get))),
        )
        .route(
            "/v1/sets/{policy}/{name}",
            put(update_set)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::Update))),
        )
        .route(
            "/v1/sets/{policy}/{name}",
            delete(delete_set)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::Delete))),
        )
        .route(
            "/v1/sets/reload",
            post(post_sets_reload)
                .route_layer(middleware::from_fn_with_state(state, auth_middleware))
                .route_layer(Extension(Scope::with(Resource::Sets, Verb::Create))),
        )
}
