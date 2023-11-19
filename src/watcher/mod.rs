use crate::context::AppContext;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn init(ctx: Arc<AppContext>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let now = tokio::time::Instant::now();
            let execute_at = std::time::Duration::from_secs(30);

            tokio::time::sleep_until(now + execute_at).await;

            execute(ctx.clone()).await.unwrap();
        }
    })
}

async fn execute(ctx: Arc<AppContext>) -> crate::Result<()> {
    dbg!(Arc::strong_count(&ctx));
    Ok(())
}
