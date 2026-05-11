use std::sync::Arc;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::AppState;
use crate::handler::*;
use crate::jwt_auth::auth;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker",  get(health_checker))
        .route("/api/auth/register", post(register_user))
        .route("/api/auth/login", post(login_user))
        .route("/api/auth/logout",get(logout_user)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .route("/api/auth/me", get(get_me)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)))
        .with_state(app_state)
}