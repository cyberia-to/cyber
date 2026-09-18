#!/bin/bash
# hourly launch worker: one sonnet run, one PR. launchd label: to.cyberia.launch-hour
set -u
export PATH="/Users/master/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"
export HOME=/Users/master
LOCK=/tmp/launch-hour.lock
LOG=/tmp/launch-hour.log
PROMPT=/Users/master/cyber/cyber/scripts/launch-hour.prompt.md
ts() { date -u +%Y-%m-%dT%H:%M:%SZ; }
if [ -d "$LOCK" ]; then
  if [ -n "$(find "$LOCK" -maxdepth 0 -mmin +120)" ]; then rm -rf "$LOCK"; else echo "$(ts) skip: run in progress" >> "$LOG"; exit 0; fi
fi
mkdir "$LOCK" || exit 0
trap 'rm -rf "$LOCK"' EXIT
mkdir -p /tmp/launch-work
for d in /Users/master/cyber/*/; do n=$(basename "$d"); [ -e "/tmp/launch-work/$n" ] || ln -s "${d%/}" "/tmp/launch-work/$n"; done
echo "$(ts) start" >> "$LOG"
cd /Users/master/cyber || exit 1
claude -p "$(cat "$PROMPT")" --model sonnet --dangerously-skip-permissions --add-dir /Users/master/cyber --add-dir /tmp/launch-work >> "$LOG" 2>&1 &
PID=$!
for _ in $(seq 1 330); do kill -0 "$PID" 2>/dev/null || break; sleep 10; done
if kill -0 "$PID" 2>/dev/null; then echo "$(ts) timeout after 55m, killing" >> "$LOG"; kill -TERM "$PID"; sleep 5; kill -KILL "$PID" 2>/dev/null; fi
echo "$(ts) end" >> "$LOG"
