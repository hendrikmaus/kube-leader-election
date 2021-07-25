_default:
  just --list --unsorted

# name for the local cluster
cluster := "kube-leader-election"

# start a local kubernetes cluster, if not running
start-test-cluster:
  #!/usr/bin/env bash
  set -euo pipefail
  if ! k3d cluster list | grep -qF "{{cluster}}"; then
    k3d cluster create "{{cluster}}" --k3s-server-arg="--no-deploy=traefik"
  else
    echo "Cluster already running"
  fi

# clean the local cluster
stop-test-cluster:
  #!/usr/bin/env bash
  set -euo pipefail
  if k3d cluster list | grep -qF "{{cluster}}"; then
    k3d cluster delete "{{cluster}}"
  else
    echo "Cluster already pruned"
  fi
