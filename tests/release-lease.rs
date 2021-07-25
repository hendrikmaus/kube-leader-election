mod lib;

use crate::lib::KubeTestUtil;
use k8s_openapi::api::coordination::v1::Lease;
use kube_leader_election::{LeaseLock, LeaseLockParams};
use std::time::Duration;

#[async_std::test]
async fn release_lease() -> anyhow::Result<()> {
    const NAMESPACE: &str = "release-lease";
    const LEASE_NAME: &str = "release-lease-test";
    const HOLDER_ID: &str = "release-lease-test-holder";

    KubeTestUtil::create_namespace(NAMESPACE)?;
    KubeTestUtil::delete_lease(NAMESPACE, LEASE_NAME)?;

    let client = kube::Client::try_default().await?;
    let leases: kube::Api<Lease> = kube::Api::namespaced(client.clone(), NAMESPACE);

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

    // release lease
    leadership.step_down().await?;

    let lease = leases.get(LEASE_NAME).await?;
    assert_eq!(
        "",
        lease
            .spec
            .as_ref()
            .unwrap()
            .holder_identity
            .as_ref()
            .unwrap()
    );

    assert_eq!(
        1,
        *lease
            .spec
            .as_ref()
            .unwrap()
            .lease_duration_seconds
            .as_ref()
            .unwrap()
    );

    async_std::task::sleep(Duration::from_secs(2)).await;

    // if we re-acquire the lease, its duration is increased from 1 to 15 again
    let lease = leadership.try_acquire_or_renew().await?.lease.unwrap();

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

    KubeTestUtil::delete_namespace(NAMESPACE)?;

    Ok(())
}
