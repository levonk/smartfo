#!/usr/bin/env bash
# mv/rm scenario matrix — exercises the smartfo binary across every
# move/remove direction (into / out of / within / cross-worktree) in both
# --plain and smart modes, on a throwaway /tmp sandbox.
#
# Assertions encode the *intended* behavior from AGENTS.md, not the current
# stub behavior. Smart mv/rm exec paths are unimplemented (stories 03-001 /
# 03-002), so those cases FAIL today and serve as a red-light regression
# suite: they go green as the implementation lands. When every case passes
# the sandbox is auto-removed; until then it is kept for inspection.
#
# Usage: scripts/tests/mv-rm-matrix.sh [--verbose]
set -euo pipefail

VERBOSE=0
[ "${1:-}" = "--verbose" ] && VERBOSE=1

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
if [ -x "$REPO_ROOT/target/release/smartfo" ]; then
    SMARTFO_BIN="$REPO_ROOT/target/release/smartfo"
elif [ -x "$REPO_ROOT/target/debug/smartfo" ]; then
    SMARTFO_BIN="$REPO_ROOT/target/debug/smartfo"
else
    echo "building smartfo (no binary found)..." >&2
    (cd "$REPO_ROOT" && cargo build --release) >&2
    SMARTFO_BIN="$REPO_ROOT/target/release/smartfo"
fi

ROOT="$(mktemp -d -t smartfo-matrix.XXXXXX)"
export HOME="$ROOT/home"
export XDG_DATA_HOME="$ROOT/xdg/data"
export XDG_CONFIG_HOME="$ROOT/xdg/config"
export XDG_CACHE_HOME="$ROOT/xdg/cache"
mkdir -p "$HOME" "$XDG_DATA_HOME" "$XDG_CONFIG_HOME" "$XDG_CACHE_HOME"

# Bin dir on PATH so `mv`/`rm` resolve to smartfo symlinks (argv[0] dispatch).
BIN="$ROOT/bin"; mkdir -p "$BIN"
ln -s "$SMARTFO_BIN" "$BIN/mv"
ln -s "$SMARTFO_BIN" "$BIN/rm"
export PATH="$BIN:$PATH"

# smartfo treats non-TTY stdin as "read paths from stdin", which drops file
# operands in any script/CI/agent context. Wrap each invocation in a PTY so
# the real argv path is exercised. ponytail: ceiling — if smartfo ever fixes
# is_stdin_piped to require an explicit `-` flag, this wrapper can be dropped.
pty() { script -q /dev/null "$@"; }

CAP="$ROOT/.capture"           # per-case process output, shown only if --verbose
run() {                        # run <cmd...> — capture, swallow exit code
    set +e
    if [ "$VERBOSE" -eq 1 ]; then pty "$@" 2>&1 | tee "$CAP" >/dev/null
    else pty "$@" >"$CAP" 2>&1; fi
    set -e
}
run_rc() {                     # run_rc <cmd...> — capture, preserve exit code
    set +e
    if [ "$VERBOSE" -eq 1 ]; then pty "$@" 2>&1 | tee "$CAP" >/dev/null
    else pty "$@" >"$CAP" 2>&1; fi
    local rc=$?; set -e; return $rc
}

PASS=0; FAIL=0; FAILED=()
begin() { CUR="$1"; : > "$CAP"; }
ok()   { printf '  PASS  %s\n' "$CUR"; PASS=$((PASS+1)); }
bad()  { printf '  FAIL  %s — %s\n' "$CUR" "${1:-?}"; FAIL=$((FAIL+1)); FAILED+=("$CUR"); }
rel() { printf '%s' "${1#"$ROOT/"}"; }   # strip sandbox prefix for concise reasons

# --- Scaffold: main repo, a linked worktree, and an outside dir ----------
REPO="$ROOT/repo"; OUT="$ROOT/outside"; WT="$ROOT/repo-wt"
mkdir -p "$OUT"
git init -q "$REPO"
git -C "$REPO" config user.name "Matrix Test"
git -C "$REPO" config user.email "matrix@test"
git -C "$REPO" config commit.gpgsign false
git -C "$REPO" -c core.hooksPath=/dev/null commit -q --allow-empty -m init
git -C "$REPO" worktree add -q -b wt-side "$WT"

# mk <dir> <name> <content> [tracked]  — create a file; if tracked, commit in its dir
mk() {
    local dir="$1" name="$2" content="$3" tracked="${4:-}"
    printf '%s' "$content" > "$dir/$name"
    if [ "$tracked" = "tracked" ]; then
        git -C "$dir" add -A
        git -C "$dir" -c core.hooksPath=/dev/null commit -q --allow-empty -m "add $name"
    fi
}

echo "mv matrix${VERBOSE:+ (verbose)}:"

# mv <mode> <ftype> <direction> :
#   within      : src+dest in $REPO
#   out-of      : src in $REPO, dest in $OUT
#   into        : src in $OUT,  dest in $REPO
#   cross-wt    : src in $REPO, dest in $WT
N=0
mv_case() {
    local mode="$1" ftype="$2" dir="$3"
    N=$((N+1)); local src="s$N.txt" dest="d$N.txt"
    begin "mv/$dir/$ftype/$mode"
    local s d
    case "$dir" in
        within)   mk "$REPO" "$src" "C" "$ftype"; s="$REPO/$src"; d="$REPO/$dest" ;;
        out-of)   mk "$REPO" "$src" "C" "$ftype"; s="$REPO/$src"; d="$OUT/$dest"  ;;
        into)     mk "$OUT"  "$src" "C";          s="$OUT/$src";  d="$REPO/$dest" ;;
        cross-wt) mk "$REPO" "$src" "C" "$ftype"; s="$REPO/$src"; d="$WT/$dest"   ;;
    esac
    ( cd "$REPO" && run mv --"$mode" --blocking "$src" "$d" )
    if [ -e "$s" ] || [ ! -e "$d" ]; then bad "file not moved ($(rel "$s") -> $(rel "$d"))"
    elif [ "$(cat "$d")" != "C" ]; then bad "content mismatch at $(rel "$d")"
    else ok; fi
}

for mode in plain smart; do
    for ftype in tracked untracked; do
        mv_case "$mode" "$ftype" within
        mv_case "$mode" "$ftype" out-of
        mv_case "$mode" "$ftype" into
        mv_case "$mode" "$ftype" cross-wt
    done
done

# Refusal gate: mv tracked out-of-repo (default smart mode) without
# --force-outside-vcs must FAIL. No --plain, no --force-outside-vcs.
N=$((N+1)); gate="gate$N.txt"
begin "mv/out-of/tracked/smart/no-force-flag"
mk "$REPO" "$gate" "G" tracked
if ( cd "$REPO" && run_rc mv --blocking "$gate" "$OUT/$gate" ) ; then
    bad "expected refusal (tracked->outside without --force-outside-vcs), got success"
else
    ok
fi

echo "rm matrix:"

# rm <mode> <ftype> <location> :
#   within   : file in $REPO
#   outside  : file in $OUT
#   worktree : file in $WT
rm_case() {
    local mode="$1" ftype="$2" loc="$3"
    N=$((N+1)); local name="r$N.txt"
    begin "rm/$loc/$ftype/$mode"
    local p cd_dir
    case "$loc" in
        within)   mk "$REPO" "$name" "C" "$ftype"; p="$REPO/$name"; cd_dir="$REPO" ;;
        outside)  mk "$OUT"  "$name" "C";          p="$OUT/$name";  cd_dir="$OUT"  ;;
        worktree) mk "$WT"   "$name" "C" "$ftype"; p="$WT/$name";   cd_dir="$WT"   ;;
    esac
    ( cd "$cd_dir" && run rm --"$mode" --blocking "$name" )
    if [ -e "$p" ]; then bad "file not removed ($(rel "$p"))"
    else ok; fi
}

for mode in plain smart; do
    for ftype in tracked untracked; do
        rm_case "$mode" "$ftype" within
        rm_case "$mode" "$ftype" outside
        rm_case "$mode" "$ftype" worktree
    done
done

# --- Summary -------------------------------------------------------------
echo
echo "summary: $PASS passed, $FAIL failed"
if [ "$FAIL" -gt 0 ]; then
    echo "failed cases:"
    for f in "${FAILED[@]}"; do printf '  - %s\n' "$f"; done
fi

if [ "$FAIL" -eq 0 ]; then
    echo "all green — cleaning up $ROOT"
    rm -rf "$ROOT"
    exit 0
else
    echo "sandbox kept at $ROOT (pass --verbose for process output)" >&2
    exit 1
fi
