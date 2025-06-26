use kube_leader_election::{LeaseLock, LeaseLockParams, LeaseLockResult};
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    env_logger::init();

    // Shared across the application to allow other parts to determine if they can act as leader
    let is_leader = Arc::new(AtomicBool::new(false));

    // Run leader election as background process
    {
        let is_leader = is_leader.clone();

        tokio::spawn(async move {
            let client = kube::Client::try_default().await.unwrap();

            // random id part for the sake of simulating something like a pod hash
            let random: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            let holder_id = format!("shared-lease-{}", random.to_lowercase());

            let leadership = LeaseLock::new(
                client,
                "default",
                LeaseLockParams {
                    holder_id,
                    lease_name: "shared-lease-example".into(),
                    lease_ttl: Duration::from_secs(15),
                },
            );

            loop {
                match leadership.try_acquire_or_renew().await {
                    Ok(ll) => is_leader.store(matches!(ll, LeaseLockResult::Acquired(_)), Ordering::Relaxed),
                    Err(err) => log::error!("{:?}", err),
                }
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    loop {
        log::info!("currently leading: {}", is_leader.load(Ordering::Relaxed));
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
