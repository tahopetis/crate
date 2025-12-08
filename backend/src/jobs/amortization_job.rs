use crate::database::{PgPool, ValuationRepository};
use crate::error::{AppError, AppResult};
use cron::Schedule;
use std::str::FromStr;
use tokio::time;
use tracing::{info, error};

pub async fn run_amortization_job(pg_pool: PgPool) -> AppResult<()> {
    let _valuation_repository = ValuationRepository::new(pg_pool);

    // TODO: Implement daily amortization calculation

    info!("Amortization job completed successfully");
    Ok(())
}

pub async fn start_amortization_scheduler(pg_pool: PgPool) -> AppResult<()> {
    let schedule = Schedule::from_str("0 2 * * *")?; // Run at 2 AM every day

    info!("Starting amortization scheduler");

    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_secs(3600)); // Check every hour

        loop {
            interval.tick().await;

            // Check if it's time to run the job
            let now = chrono::Utc::now();
            if schedule.upcoming(chrono::Utc).next().unwrap_or(now + chrono::Duration::hours(24)) <= now + chrono::Duration::hours(1) {
                if let Err(e) = run_amortization_job(pg_pool.clone()).await {
                    error!("Error running amortization job: {:?}", e);
                }
            }
        }
    });

    Ok(())
}