use filaco::{Filaco, FilacoResult};

#[tokio::main]
async fn main() -> FilacoResult<()> {
    Filaco::default().run().await
}
