use cmd_lib::run_cmd;

pub struct KubeTestUtil;

impl KubeTestUtil {
    /// Create a Kubernetes namespace, if it doesn't exist
    pub fn create_namespace(namespace: &str) -> anyhow::Result<()> {
        if let Err(_) = run_cmd!(kubectl get namespace ${namespace}) {
            run_cmd!(kubectl create namespace ${namespace} > /dev/null 2>&1)?;
        }

        Ok(())
    }

    /// Delete a Kubernetes namespace, if it exists, and exit immediately, without waiting for completion
    pub fn delete_namespace(namespace: &str) -> anyhow::Result<()> {
        if let Ok(_) = run_cmd!(kubectl get namespace ${namespace}) {
            run_cmd!(kubectl delete namespace ${namespace} --wait=false > /dev/null 2>&1)?;
        }

        Ok(())
    }

    /// Delete a Kubernetes Lease, if it exists
    pub fn delete_lease(namespace: &str, lease_name: &str) -> anyhow::Result<()> {
        if let Ok(_) = run_cmd!(kubectl --namespace ${namespace} get lease ${lease_name}) {
            run_cmd!(kubectl --namespace ${namespace} delete lease ${lease_name} > /dev/null 2>&1)?;
        }

        Ok(())
    }
}
