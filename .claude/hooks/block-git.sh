#!/bin/sh
# PreToolUse hook for Bash: agents may only READ the repository with git.
#
# Allowed:  git status, diff, log, show, ls-files, blame (optionally after -C <dir>)
# Denied:   every other git subcommand (add, commit, push, checkout, stash, ...),
#           including inside compound commands such as `cargo test && git add -A`.
#
# Agents inspect the working tree to draft a commit message; the human runs git.
cmd=$(jq -r '.tool_input.command // ""')

# No git invocation at all: nothing to do.
if ! printf '%s' "$cmd" | grep -Eq '(^|[;&|(`[:space:]])git([[:space:]]|$)'; then
	exit 0
fi

# Every git invocation must match the read-only allowlist.
readonly_re='(^|[;&|(`[:space:]])git([[:space:]]+-C[[:space:]]+[^[:space:]]+)?[[:space:]]+(status|diff|log|show|ls-files|blame)([[:space:]]|$)'
total=$(printf '%s' "$cmd" | grep -oE '(^|[;&|(`[:space:]])git([[:space:]]|$)' | wc -l | tr -d ' ')
allowed=$(printf '%s' "$cmd" | grep -oE "$readonly_re" | wc -l | tr -d ' ')

if [ "$total" -eq "$allowed" ]; then
	exit 0
fi

jq -n '{
  hookSpecificOutput: {
    hookEventName: "PreToolUse",
    permissionDecision: "deny",
    permissionDecisionReason: "This repo only lets agents run read-only git (status, diff, log, show, ls-files, blame). Suggest a commit message instead; the user runs git themselves."
  }
}'
exit 0
