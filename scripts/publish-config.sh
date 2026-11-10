#!/usr/bin/env bash
# 把远程配置同步到自有域名（客户端首选这个源）。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/../speed-test-service/public/app"

mkdir -p "$DEST"
cp "$ROOT/docs/update.js" "$DEST/update.js"
chmod 644 "$DEST/update.js"

echo "已发布: $DEST/update.js"
curl -fsS --max-time 10 "https://svip.nowuse-ai.com/app/update.js?t=$(date +%s)" | grep -E "min:|entries" | head -3
