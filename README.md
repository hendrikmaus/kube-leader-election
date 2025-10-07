# Kubernetes Leader Election in Rust

![CI workflow](https://github.com/hendrikmaus/kube-leader-election/actions/workflows/ci.yaml/badge.svg)
![crates.io version](https://img.shields.io/crates/v/kube-leader-election)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This library provides simple leader election for Kubernetes workloads.

<!-- x-release-please-start-version -->
```toml
[dependencies]
kube-leader-election = "0.42.0"
```
<!-- x-release-please-end -->

## Example

Acquire leadership on a Kubernetes [`Lease`](https://kubernetes.io/docs/reference/kubernetes-api/cluster-resources/lease-v1/) called `some-operator-lock`, in the `default` namespace and promise to renew the lock every 15 seconds:

```rust
let leadership = LeaseLock::new(
    kube::Client::try_default().await?,
    "default",
    LeaseLockParams {
        holder_id: "some-operator".into(),
        lease_name: "some-operator-lock".into(),
        lease_ttl: Duration::from_secs(15),
    },
);

// Run this in a background task every 5 seconds
// Share the result with the rest of your application; for example using Arc<AtomicBool>
// See https://github.com/hendrikmaus/kube-leader-election/blob/master/examples/shared-lease.rs
let lease = leadership.try_acquire_or_renew().await?;

log::info!("currently leading: {}", lease.acquired_lease);
```

*Please refer to the [`examples`](https://github.com/hendrikmaus/kube-leader-election/tree/master/examples) for runnable usage demonstrations.*

## Features

- Kubernetes `Lease` locking, similar to [client-go's leaderelection](https://pkg.go.dev/k8s.io/client-go/tools/leaderelection)

## Kubernetes `Lease` Locking

A very basic form of leader election without fencing, i.e., only use this if your application can tolerate multiple replicas acting as leader for a short amount of time.

This implementation uses a Kubernetes `Lease` resource from the API group `coordination.k8s.io`, which is locked and continuously renewed by the leading replica. The leaseholder, as well as all candidates, use timestamps to determine if a lease can be acquired. Therefore, this implementation is volatile to datetime skew within a cluster.

Only use this implementation if you are aware of its downsides, and your workload can tolerate them.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)