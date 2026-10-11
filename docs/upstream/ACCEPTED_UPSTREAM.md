# Accepted upstream changes

Inherited baseline: `995c768dcba4da7f93302bf2107af6b7ec3df02b`.

## Abrupt-session resume and history safety

Upstream: `f5a946c6bb1b332a1147153821172cf08eb84d30` (Spotifast #727).
Port commit: `48801ff` on feature/upstream-safety, preserving Dennis's authorship.
Applied without source adaptations. Track changes mark the session dirty and
active playback refreshes resume state every 30 seconds. History uses a synced
temporary file and atomic replacement; failed writes remain dirty for retry.
JSON formats, credentials, queue behavior, audio and appearance are unchanged.

Regression coverage includes periodic resume saving, whole-file replacement,
failed-write retry, and Unix owner-only permissions. Branch-specific validation
and protected CI must pass before promotion. Unix behavior requires platform CI;
a Windows run does not validate Unix filesystem behavior.

Future entries must record source SHA, author, purpose, adaptations, port commit,
verification and any platform or performance limitations.

## Branch validation, 2026-10-11

On Windows x64, formatting, both Clippy configurations with warnings denied,
default/all-feature full suites and documentation tests passed. Library totals
were 942 and 965 passed, each with one existing credential-store test ignored.
Integration suites passed after the exact neutral executable-name renames.
The optimized build passed in 2m 27s. The upstream inventory smoke reported
one accepted commit and 16 pending commits; GitHub's report job also passed.
Other platform and protected checks remain pending on PR #1. No live playback
or new performance measurement is claimed for this port.
