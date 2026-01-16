use axum::{
    Extension, Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};
use tracing::{error, info};

use crate::extractors::{ValidatedJson, ValidatedPath};
use common::{Client, ClientName};

use super::AppState;
use super::middleware::auth_middleware;
use super::rbac::{Resource, Scope, Verb};

// List all clients
async fn list_clients(State(state): State<AppState>) -> impl IntoResponse {
    let clients = state.rbac_service.read().await.list_clients();
    (StatusCode::OK, Json(clients)).into_response()
}

// Create a new client
async fn create_client(
    State(state): State<AppState>,
    Extension(actor): Extension<Client>,
    ValidatedJson(client): ValidatedJson<Client>,
) -> impl IntoResponse {
    match state.rbac_service.write().await.add_client(&client) {
        Ok(new_client) => {
            info!(
                actor.name = %actor.name,
                actor.group = %actor.group,
                created.name = %new_client.name,
                created.group = %new_client.group,
                "client created"
            );
            (StatusCode::CREATED, Json(new_client)).into_response()
        }
        Err(ce) => {
            error!(
                actor.name = %actor.name,
                actor.group = %actor.group,
                error = %ce,
                "failed to create client"
            );
            ce.into_response()
        }
    }
}

// Inspect a client by name
async fn get_client(
    State(state): State<AppState>,
    ValidatedPath(name): ValidatedPath<ClientName>,
) -> impl IntoResponse {
    match state.rbac_service.read().await.get_client_by_name(&name) {
        Ok(client) => (StatusCode::OK, Json(client)).into_response(),
        Err(ce) => ce.into_response(),
    }
}

// Delete a client by name
async fn delete_client(
    State(state): State<AppState>,
    ValidatedPath(name): ValidatedPath<ClientName>,
    Extension(actor): Extension<Client>,
) -> impl IntoResponse {
    match state
        .rbac_service
        .write()
        .await
        .delete_client_by_name(&name)
    {
        Ok(()) => {
            info!(
                actor.name = %actor.name,
                actor.group = %actor.group,
                deleted.name = %name,
                "client deleted"
            );
            StatusCode::NO_CONTENT.into_response()
        }
        Err(ce) => {
            error!(
                actor.name = %actor.name,
                actor.group = %actor.group,
                deleted.name = %name,
                error = %ce,
                "failed to delete client"
            );
            ce.into_response()
        }
    }
}

pub(super) fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/v1/client",
            get(list_clients)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Clients, Verb::List))),
        )
        .route(
            "/v1/client",
            post(create_client)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Clients, Verb::Create))),
        )
        .route(
            "/v1/client/{name}",
            get(get_client)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth_middleware,
                ))
                .route_layer(Extension(Scope::with(Resource::Clients, Verb::Get))),
        )
        .route(
            "/v1/client/{name}",
            delete(delete_client)
                .route_layer(middleware::from_fn_with_state(state, auth_middleware))
                .route_layer(Extension(Scope::with(Resource::Clients, Verb::Delete))),
        )
}
