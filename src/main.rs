use axum::extract::FromRef;
use axum::routing::get;
use axum::{serve, Router};
use filaco::{portfolio, CommandHandler, FilacoSocketAddr, QueryHandler};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use yadi::Container;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Container::builder()
        .register_lazy::<FilacoSocketAddr>()
        .register_lazy::<CommandHandler>()
        .register_lazy::<QueryHandler>()
        .build()?;

    let addr: SocketAddr = (*config.resolve::<FilacoSocketAddr>()).into();
    let listener = TcpListener::bind(addr).await?;

    let state = FilacoState(Arc::new(config));

    let app = Router::new().route("/", get(portfolio)).with_state(state);

    serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct FilacoState(Arc<Container>);

impl FromRef<FilacoState> for Arc<Container> {
    fn from_ref(input: &FilacoState) -> Self {
        input.0.clone()
    }
}

impl FromRef<FilacoState> for Arc<CommandHandler> {
    fn from_ref(input: &FilacoState) -> Self {
        input.0.resolve::<CommandHandler>()
    }
}

impl FromRef<FilacoState> for Arc<QueryHandler> {
    fn from_ref(input: &FilacoState) -> Self {
        input.0.resolve::<QueryHandler>()
    }
}
