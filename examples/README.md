# Leader Election Examples

All examples require access to a Kubernetes cluster.

You can create a local cluster from the root using `just start-test-cluster`; requires [`docker`](https://docker.com), [`just`](https://github.com/casey/just) and [`k3d`](https://k3d.io).

## Run an Example

```shell
cargo run --example <name>
```

## Simple Lease - `simple-lease.rs`

This example creates a Kubernetes `Lease` based locking mechanism, acquires leadership and steps down after a few seconds.

## Shared Lease - `shared-lease.rs`

A more sophisticated usage example with the leader election running in a background process that updates an `AtomicBool` inside an `Arc`.

Open up two shells and start a replica of the example in each of them. The first one will acquire the lock and be leading. The second one will be a follower. Now try to terminate the leading replica and wait at least 15 seconds to see the following acquiring the leadership.
