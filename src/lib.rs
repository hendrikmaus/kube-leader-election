//! Crate for implementing leader election in Kubernetes operators
//!
//! # Example
//!
//! The following example uses a Kubernetes `Lease` to implement leader election, acquires leadership,
//! waits a little while and steps down again.
//!
//! ```rust,no_run
//!use kube_leader_election::{LeaseLock, LeaseLockParams};
//!use std::time::Duration;
//!
//!#[tokio::main]
//!async fn main() -> anyhow::Result<()> {
//!    std::env::set_var(
//!        "RUST_LOG",
//!        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
//!    );
//!    env_logger::init();
//!
//!    // Configure the LeaseLock mechanism
//!    //
//!    // One should try to renew/acquire the lease before `lease_ttl` runs out.
//!    // E.g. if `lease_ttl` is set to 15 seconds, one should renew it every 5 seconds.
//!    let leadership = LeaseLock::new(
//!        kube::Client::try_default().await?,
//!        "default",
//!        LeaseLockParams {
//!            holder_id: "simple-lease".into(),
//!            lease_name: "simple-lease-example".into(),
//!            lease_ttl: Duration::from_secs(15),
//!        },
//!    );
//!
//!    // Run this in a background task and share the result with the rest of your application
//!    let _lease = leadership.try_acquire_or_renew().await?;
//!    // `lease.acquired_lease` can be used to determine if we're leading or not
//!
//!    log::info!("waiting 5 seconds, then stepping down again");
//!    tokio::time::sleep(Duration::from_secs(5)).await;
//!
//!    // To give up leadership, call `step_down`.
//!    //
//!    // This will set the current ttl on the lease to 1s and remove the current holder identity,
//!    // so all other candidates start to race for the lock on the lease.
//!    leadership.step_down().await?;
//!
//!    Ok(())
//!}
//!```
//!
//! Please refer to the [examples](./examples) for more details.

#![deny(unsafe_code)]

mod lease;

pub use lease::{Error, LeaseLock, LeaseLockParams, LeaseLockResult};
