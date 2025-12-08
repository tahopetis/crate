use crate::database::PgPool;
use crate::jobs::{run_amortization_job, run_cleanup_job};
use crate::error::{AppError, AppResult};
use tokio::time;
use tracing::{info, error};
use chrono::Timelike;

pub async fn start_background_jobs(pg_pool: PgPool) -> AppResult<()> {
    info!("Starting background jobs scheduler");

    // Start amortization job (daily at 2 AM)
    let amortization_pool = pg_pool.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(3600)); // Check every hour

        loop {
            interval.tick().await;

            let now = chrono::Utc::now();
            if now.hour() == 2 && now.minute() == 0 {
                if let Err(e) = run_amortization_job(amortization_pool.clone()).await {
                    error!("Error running amortization job: {:?}", e);
                }
            }
        }
    });

    // Start cleanup job (daily at 3 AM)
    let cleanup_pool = pg_pool.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(3600)); // Check every hour

        loop {
            interval.tick().await;

            let now = chrono::Utc::now();
            if now.hour() == 3 && now.minute() == 0 {
                if let Err(e) = run_cleanup_job(cleanup_pool.clone()).await {
                    error!("Error running cleanup job: {:?}", e);
                }
            }
        }
    });

    info!("Background jobs scheduler started");
    Ok(())
}