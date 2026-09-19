#!/bin/bash
# launch workers: three parallel slots (lanes A, B, C), each one sonnet run with its own mirror and lock.
# launchd label: to.cyberia.launch-hour, StartInterval 1200. usage: launch-hour.sh [slot...]
set -u
export PATH="/Users/master/.cargo/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"
export HOME=/Users/master
PROMPT=/Users/master/cyber/cyber/scripts/launch-hour.prompt.md
ts() { date -u +%Y-%m-%dT%H:%M:%SZ; }
mkdir -p /tmp/launch-claims
run_slot() {
  local slot=$1 lane=$2
  local lock=/tmp/launch-$slot.lock log=/tmp/launch-$slot.log mirror=/tmp/launch-work-$slot
  if [ -d "$lock" ]; then
    if [ -n "$(find "$lock" -maxdepth 0 -mmin +120)" ]; then rm -rf "$lock"; else echo "$(ts) skip: run in progress" >> "$log"; return 0; fi
  fi
  mkdir "$lock" || return 0
  mkdir -p "$mirror"
  for d in /Users/master/cyber/*/; do n=$(basename "$d"); [ -e "$mirror/$n" ] || ln -s "${d%/}" "$mirror/$n"; done
  echo "$(ts) start slot=$slot lane=$lane" >> "$log"
  ( cd /Users/master/cyber && exec claude -p "lane preference: $lane · mirror: $mirror/ · slot: $slot
$(cat "$PROMPT")" --model sonnet --dangerously-skip-permissions --add-dir /Users/master/cyber --add-dir "$mirror" >> "$log" 2>&1 ) &
  local pid=$!
  for _ in $(seq 1 330); do kill -0 "$pid" 2>/dev/null || break; sleep 10; done
  if kill -0 "$pid" 2>/dev/null; then echo "$(ts) timeout after 55m, killing" >> "$log"; pkill -TERM -P "$pid" 2>/dev/null; kill -TERM "$pid" 2>/dev/null; sleep 5; pkill -KILL -P "$pid" 2>/dev/null; kill -KILL "$pid" 2>/dev/null; fi
  for r in /Users/master/cyber/*/; do n=$(basename "$r"); if [ -d "$mirror/$n" ] && [ ! -L "$mirror/$n" ]; then git -C "$r" worktree remove --force "$mirror/$n" 2>/dev/null; rm -rf "$mirror/$n"; ln -s "${r%/}" "$mirror/$n"; echo "$(ts) reclaimed worktree $n" >> "$log"; fi; done
  echo "$(ts) end slot=$slot" >> "$log"
  rm -rf "$lock"
}
slots="${*:-a b c}"
for s in $slots; do
  case $s in a) lane="A core";; b) lane="B body";; c) lane="C content";; *) lane="A core";; esac
  nohup bash -c "$(declare -f ts run_slot); PROMPT=$PROMPT; run_slot $s '$lane'" >/dev/null 2>&1 &
  disown
done
exit 0
