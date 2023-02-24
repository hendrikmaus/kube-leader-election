mod utils;

use crate::utils::KubeTestUtil;
use k8s_openapi::api::coordination::v1::Lease;
use kube_leader_election::{LeaseLock, LeaseLockParams};
use std::time::Duration;

#[tokio::test]
async fn create_lease() -> anyhow::Result<()> {
    const NAMESPACE: &str = "create-lease";
    const LEASE_NAME: &str = "create-lease-test";
    const HOLDER_ID: &str = "create-lease-test-holder";

    KubeTestUtil::create_namespace(NAMESPACE)?;
    KubeTestUtil::delete_lease(NAMESPACE, LEASE_NAME)?;

    let client = kube::Client::try_default().await?;
    let leases: kube::Api<Lease> = kube::Api::namespaced(client.clone(), NAMESPACE);

    // start by asserting that we have no lease resource in the cluster
    let lease = leases.get(LEASE_NAME).await;
    assert!(lease.is_err());

    // next create a lease
    let leadership = LeaseLock::new(
        client.clone(),
        NAMESPACE,
        LeaseLockParams {
            holder_id: HOLDER_ID.into(),
            lease_name: LEASE_NAME.into(),
            lease_ttl: Duration::from_secs(15),
        },
    );
    leadership.try_acquire_or_renew().await?;

    // assert that the lease was created
    let lease = leases.get(LEASE_NAME).await?;
    assert_eq!(LEASE_NAME, lease.metadata.name.unwrap());
    assert_eq!(
        HOLDER_ID,
        lease
            .spec
            .as_ref()
            .unwrap()
            .holder_identity
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        15,
        *lease
            .spec
            .as_ref()
            .unwrap()
            .lease_duration_seconds
            .as_ref()
            .unwrap()
    );
    assert_eq!(
        0,
        *lease
            .spec
            .as_ref()
            .unwrap()
            .lease_transitions
            .as_ref()
            .unwrap()
    );

    KubeTestUtil::delete_namespace(NAMESPACE)?;

    Ok(())
}
