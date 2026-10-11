# Upstream baseline

SpotiPhile inherits Spotifast at `995c768dcba4da7f93302bf2107af6b7ec3df02b`.
This immutable baseline is retained for comparisons, attribution and review.

Origin is https://github.com/DaisyCatTs/SpotiPhile. Upstream is
https://github.com/crmne/spotifast. On 2026-10-11 the reviewed upstream tip was
`dd2d5e34e66d4e01e31e595a3022a1de808b449a`, 17 commits after the baseline.

The owner authorized validated promotion to main on 2026-10-11. Use focused
branches and protected pull requests. Preserve SpotiPhile features, MilkDrop,
Winamp and native Rust rendering. Review and port useful upstream changes with
regression coverage and author credit. Do not merge upstream wholesale or rewrite
published history. UI redesign remains paused and requires separate review.

The daily read-only workflow reports pending commits at 08:17 UTC once present
on the default branch. It cannot integrate code, publish or modify the repository.
GitHub may delay scheduled runs. The report is an inventory, not automatic approval.

## Solo-maintainer promotion policy

On 2026-10-11 the owner requested merging completed work without a separate
write-access reviewer. Main now requires zero approving reviews; all seven
required status checks, strict up-to-date checks, enforced admin protection,
linear history, and disabled force pushes/deletion remain intact.
PR #1 merged as c27e5c25c0c1f2d7e6bc6022ff87b34047fc7328.
Squashed stack ancestry conflicted on retargeting #2. Its two source ports were
replayed on current main without rewriting published branches. Source, tests,
Cargo files, workflows and README are identical to the previously CI-validated
981a653 snapshot. Fresh protected CI remains required on this new ancestry.
