use crate::QueryHandler;
use axum::extract::State;
use std::sync::Arc;

pub async fn portfolio(State(query_handler): State<Arc<QueryHandler>>) {}
