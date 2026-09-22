---
name: commit-message
description: Draft a commit message from the actual working tree changes (git status and git diff), without staging or committing. Use at the end of every session that changed files, or when the user asks for a commit message.
---

# Commit message

Agents in this repo never modify git state. The user commits by hand, so the agent's job is to hand them a ready-to-paste message that describes **what is actually in the working tree**, not what the agent remembers doing.

## Steps

1. **Inspect the working tree with read-only git.** These are the only git commands allowed here:

   ```sh
   git status --short
   git diff
   git diff --cached
   ```

   `git status` lists modified and untracked files. `git diff` shows unstaged changes to tracked files; `git diff --cached` shows anything already staged. Untracked files do not appear in a diff, so `cat` any new file that `git status` marks `??`. Never run `git add`, `git commit`, `git stash`, or any other command that changes state; the repo hook will block them.

2. **One message for everything uncommitted.** Staged, unstaged, and untracked changes all go into a single message. Do not split by session or by topic, and do not propose multiple commits. If the tree contains changes you did not make, they belong in the message too.

3. **Pick the prefix** from the repo's history convention:
   - `feat/` new user-visible behavior
   - `fix/` bug fix or correction
   - `chore/` build, CI, docs, tooling, config

4. **Write the subject line**: `<prefix> <lowercase summary>`, under 72 characters, imperative mood, no trailing period. Example from history: `feat/ added player preview`.

5. **Add a short body** that describes the uncommitted changes: two to five plain sentences or bullets, one per area touched, naming what changed and why. Keep it under about 60 words.

6. **Check the README rule.** If the diff touches `src/player.rs`, `src/app.rs`, `src/ui.rs`, or `src/main.rs` in a user-visible way and `README.md` or `CHANGELOG.md` are not in the diff, run the `readme-sync` skill first, then come back here.

## Output

Print exactly one message in a single fenced code block so it can be copied as-is. Nothing else: no `git add`, no `git commit`, no shell command of any kind. The user knows how to commit.

```
feat/ short summary

Short body describing the uncommitted changes.
```
