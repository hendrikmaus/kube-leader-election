# Kubernetes Leader Election in Rust

![CI workflow](https://github.com/hendrikmaus/kube-leader-election/actions/workflows/ci.yaml/badge.svg)
![crates.io version](https://img.shields.io/crates/v/kube-leader-election)

A crate to allow for creating leader election concepts for workloads running in Kubernetes clusters using Rust.

```rust
let leadership = LeaseLock::new(
    kube::Client::try_default().await?,
    "default",
    LeaseLockParams {
        holder_id: "simple-lease".into(),
        lease_name: "simple-lease-example".into(),
        lease_ttl: Duration::from_secs(15),
    },
);

// Run this in a background task every 5 seconds and share the result with the rest of your application; for example using Arc<AtomicBool>
let lease = leadership.try_acquire_or_renew().await?;
log::info!("currently leading: {}", lease.acquired_lease);
```

*Please refer to the [`examples`](./examples) for runnable usage demonstrations.*

## Target Audience

If you are creating highly available Kubernetes operators, usually only one replica must be in charge of mutating the cluster's state. This can be achieved by using a leader election mechanism, which this crate aims to provide.

## Features

- Kubernetes `Lease` locking, similar to [client-go's leaderelection](https://pkg.go.dev/k8s.io/client-go/tools/leaderelection)

## Kubernetes `Lease` Locking

A very basic form of leader election without fencing, i.e., only use this if your application can tolerate multiple replicas acting as leader for a short amount of time.

This implementation uses a Kubernetes `Lease` resource from the API group `coordination.k8s.io`, which is locked and continuously renewed by the leading replica. The leaseholder, as well as all candidates, use timestamps to determine if a lease can be acquired. Therefore, this implementation is volatile to datetime skew within a cluster.

Only use this implementation if you are aware of its downsides, and your workload can tolerate them.
