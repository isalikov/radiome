#!/bin/sh
# PreToolUse hook: deny any Bash command that invokes git, including compound
# commands such as `cd dir && git status`. Agents suggest commit messages; the
# human runs git.
cmd=$(jq -r '.tool_input.command // ""')
if printf '%s' "$cmd" | grep -Eq '(^|[;&|(`[:space:]])git([[:space:]]|$)'; then
  jq -n '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: "This repo forbids agents from running git. Suggest a commit message instead; the user runs git themselves."
    }
  }'
fi
exit 0
