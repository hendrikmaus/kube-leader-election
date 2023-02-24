mod utils;

use crate::utils::KubeTestUtil;
use kube_leader_election::{LeaseLock, LeaseLockParams};
use std::time::Duration;

#[tokio::test]
async fn leader_election() -> anyhow::Result<()> {
    const NAMESPACE: &str = "leader-election";
    const LEASE_NAME: &str = "leader-election-test";
    const HOLDER_ID_01: &str = "leader-election-test-holder01";
    const HOLDER_ID_02: &str = "leader-election-test-holder02";

    KubeTestUtil::create_namespace(NAMESPACE)?;
    KubeTestUtil::delete_lease(NAMESPACE, LEASE_NAME)?;

    let client = kube::Client::try_default().await?;

    let leadership_01 = LeaseLock::new(
        client.clone(),
        NAMESPACE,
        LeaseLockParams {
            holder_id: HOLDER_ID_01.into(),
            lease_name: LEASE_NAME.into(),
            lease_ttl: Duration::from_secs(15),
        },
    );
    leadership_01.try_acquire_or_renew().await?;

    // HOLDER_ID_01 is now leading, so HOLDER_ID_02 should follow

    let leadership_02 = LeaseLock::new(
        client.clone(),
        NAMESPACE,
        LeaseLockParams {
            holder_id: HOLDER_ID_02.into(),
            lease_name: LEASE_NAME.into(),
            lease_ttl: Duration::from_secs(15),
        },
    );
    let res = leadership_02.try_acquire_or_renew().await?;
    assert_eq!(false, res.acquired_lease);

    // now HOLDER_ID_01 will release the lock

    leadership_01.step_down().await?;

    // since the new lease ttl is 1 second, we should be able to acquire it if we wait 2s
    tokio::time::sleep(Duration::from_secs(2)).await;

    let res = leadership_02.try_acquire_or_renew().await?;
    assert!(res.acquired_lease);

    KubeTestUtil::delete_namespace(NAMESPACE)?;

    Ok(())
}
