#!/usr/bin/env bash
set -euo pipefail

gh release create "${NEW_VERSION}" --title "${NEW_VERSION}" --target "${NEW_VERSION}" --generate-notes
