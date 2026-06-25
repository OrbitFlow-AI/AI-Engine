# Wave Program Contributor Onboarding

This guide is the quick entry point for Stellar Wave Program contributors who want to build against AI-Engine during an active sprint.

AI-Engine is a Soroban/Stellar monorepo with Rust contracts, a TypeScript SDK, and documentation under `docs/`. Keep Wave contributions small, issue-scoped, and easy for maintainers to review.

## Clone -> check -> test flow

1. Fork the repository on GitHub.
2. Clone your fork and enter the repo:

   ```bash
   git clone https://github.com/<your-user>/AI-Engine.git
   cd AI-Engine
   ```

3. Add the upstream remote:

   ```bash
   git remote add upstream https://github.com/OrbitFlow-AI/AI-Engine.git
   git fetch upstream
   ```

4. Verify the Rust workspace metadata:

   ```bash
   cargo check --workspace
   ```

5. Install and test the TypeScript workspace when your issue touches SDK, docs examples, or package scripts:

   ```bash
   npm install
   npm test
   ```

6. For documentation-only issues, include the exact files reviewed in your PR validation notes instead of running unrelated build steps.

## Claim a Wave issue

1. Open the Issues tab and filter by `good-first-issue` for the smallest scoped tasks.
2. Confirm that the issue is open, unassigned, and not already covered by an active pull request.
3. If the issue is part of a Wave cycle, apply through the Wave flow or comment with a short implementation plan.
4. Wait for maintainer assignment before investing in larger code changes. For very small docs fixes, keep the PR narrow and reference the issue clearly.
5. Create a topic branch named after the issue, for example:

   ```bash
   git checkout -b docs/wave-onboarding-46
   ```

## Open a focused PR

Use one pull request per issue. In the PR body, include:

- `Closes #<issue-number>`.
- A short summary of the changed files.
- Manual validation steps such as `cargo check --workspace`, `npm test`, or documentation review.
- Screenshots only when the change affects visible UI.
- A note that no secrets, `.env` files, or generated build artifacts were committed.

Before requesting review, sync with upstream `main` and keep the diff focused on the issue acceptance criteria.

## Recommended first issues

Start with issues carrying the `good-first-issue` label. At the time this guide was added, the best small entry points were:

| Issue | Focus | Why it is a good first task |
|-------|-------|-----------------------------|
| [#47](https://github.com/OrbitFlow-AI/AI-Engine/issues/47) | README links and typos | Small docs-only cleanup. |
| [#48](https://github.com/OrbitFlow-AI/AI-Engine/issues/48) | SDK mock-mode env snippet | Documentation update tied to SDK configuration. |
| [#49](https://github.com/OrbitFlow-AI/AI-Engine/issues/49) | GitHub label conventions | Helps maintainers and contributors triage Wave work. |
| [#50](https://github.com/OrbitFlow-AI/AI-Engine/issues/50) | CODEOWNERS stubs | Small repo-governance change with clear review paths. |
| [#51](https://github.com/OrbitFlow-AI/AI-Engine/issues/51) | Markdown spellcheck config | Dev tooling task with limited blast radius. |

If those are taken, use the same filters and prefer issues that are labeled `phase-0-scaffold`, `documentation`, or `devops` before moving into contract or SDK logic.

## Maintainer expectations

Maintainers should use assignment to signal ownership, keep issue labels current, and close or update stale Wave applications before the sprint ends. Contributors should respond quickly to review comments and avoid expanding the scope after review starts.