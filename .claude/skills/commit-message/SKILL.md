---
name: commit-message
description: Draft a commit message for the changes made in this session, without running git. Use at the end of every session that changed files, or when the user asks for a commit message.
---

# Commit message

Agents in this repo never run git. The user commits by hand, so the agent's job is to hand them a ready-to-paste message.

## Steps

1. **List what changed this session** from your own memory of the edits you made: files created, modified, renamed, or deleted. Do not run `git status` or `git diff`. If you are unsure whether a file changed, `cat` it or compare against what you read earlier.
2. **Pick the prefix** from the repo's history convention:
   - `feat/` new user-visible behavior
   - `fix/` bug fix or correction
   - `chore/` build, CI, docs, tooling, config
3. **Write the subject line**: `<prefix> <lowercase summary>`, under 72 characters, imperative mood, no trailing period. Example from history: `feat/ added player preview`.
4. **Add a body only if needed**: one short paragraph or a few bullets on the *why* when the subject alone would leave a reviewer guessing. Skip it for trivial changes.
5. **Remind about README and CHANGELOG** if any change touched the player (`src/player.rs`, `src/app.rs`, `src/ui.rs`, `src/main.rs`) and those files were not updated in the same session.

## Output

Print the message in a fenced code block so it can be copied as-is, followed by the exact command the user can run:

```
feat/ short summary

Optional body.
```

```sh
git add -A && git commit -m "feat/ short summary"
```

Do not run the command yourself.
