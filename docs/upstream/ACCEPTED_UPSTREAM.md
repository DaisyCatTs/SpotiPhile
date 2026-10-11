# Accepted upstream changes

## Inherited baseline

The complete tree at `995c768dcba4da7f93302bf2107af6b7ec3df02b` is the inherited
starting point, not a claim that every inherited choice is approved for Daisy
forever. The baseline contains v0.12.0 plus the eleven commits listed in
[evidence/since-v0.12.0.txt](evidence/since-v0.12.0.txt).

## Deliberate integrations after baseline

Phase 0 fetched upstream without integrating changes. The first deliberate port
was validated on 2026-10-11:

### Abrupt-session resume and history safety, Spotifast #727

- Upstream: `crmne/spotifast`, SHA
  `f5a946c6bb1b332a1147153821172cf08eb84d30`.
- Daisy commit: `4c62229912c0c1f8f53f62df247c21ba9e44cb4d`.
- Classification: persistence reliability/privacy. Original author credited in
  the port commit; no upstream merge or history rewrite.
- Behavior: track changes mark the session dirty; playing refreshes the saved
  resume point every 30 seconds. Listening history is written through a synced
  temporary file and atomic replacement. Failed writes remain dirty for retry.
- Files: focused app scheduling/test hunks and `src/history.rs`. JSON formats,
  credentials, queue edit semantics, MilkDrop and Winamp are unchanged.
- Adaptation: app hunks applied around existing Daisy queue work; source-location
  references regenerated in catalogs without changing translations.
- Validation: both Windows Clippy configurations, default/all-features suites,
  docs checks, release build and version smoke passed. New port tests include
  periodic resume saving, whole-file replacement and failed-write retry. Windows
  totals: 1,004 / 1,027 library tests; one existing credential-store test ignored.
  The Unix owner-only permission regression is retained for CI, not claimed as
  locally executed. No new performance benchmark is claimed.

For each future accepted change record: upstream repository and full SHA, problem,
classification, chosen files/behavior, adaptations, Daisy commit, tests/platform
coverage, performance evidence when relevant, and review date. Security/auth and
playback reliability receive priority; UI logic must preserve Daisy visual scope.
