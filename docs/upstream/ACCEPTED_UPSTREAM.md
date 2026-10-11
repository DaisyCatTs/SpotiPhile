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

### Decoder wakeups and output recovery, Spotifast #726/#731

- Upstream: `277a4fabcd2ca11ced4adb45b274f29084fde7eb` and
  `13876a8e81c7abd90266a2c20d030c3e5958213b`.
- SpotiPhile queue-feature commits: `52f29af` and `ed30344`.
- Focused promotion branch commits: `18db688` and `87975a8`, PR #2 stacked on #1.
- Exact source ports, preserving Dennis's authorship. Only src/sink.rs changes.
- The decoder waits for queue consumption with a 50 ms fallback instead of
  polling every 10 ms. Queue bounds and existing seek/transition handling remain.
- Failed outputs are maintained/reopened; an output with no consumption for two
  seconds is reopened. Suspension gaps reset the stall timer. Reopening failures
  remain explicit errors, and volume is reapplied after recovery.
- The focused branch passed 28 sink tests, full Windows suites (946/969 library
  tests plus integration tests), both Clippy configurations and a release build
  in 2m 23s. Combined queue-feature verification is recorded separately in
  docs/features/upstream-audio-validation.md. Cross-platform CI remains pending.
- No new benchmark, authenticated playback, Bluetooth switch or sleep/wake
  hardware validation is claimed. Upstream's measurements are upstream evidence.

### Resampler, name-cache and ordering ports, Spotifast #681/#744/#717/#701

| Upstream SHA | Queue-feature commit | Promotion commit | Author |
| --- | --- | --- | --- |
| `031e8c565c29bd72763bf29f0cfbe17c3446bc58` | `2229f97` | `577d24a` | Dennis |
| `9dd23c9b27a0ad6d43d4569a36547f9806945451` | `6cad104` | `e985796` | Luís Fernando |
| `9d74afe8ad5196eb67942f449d6ed57988d58697` | `f420bb7` | `4be253a` | Лев Федорец |
| `5b7a5dfe8c15542ee82fd7560c1e36496306b6c9` | `8ea6ffa` | `8240528` | Samet Karaş |

Applied without source adaptations, retaining original author/co-author metadata.
Stereo resampling uses portable split sums with numerical regression coverage.
Overlapping failed lookups do not replace known adder names, while successful
renames still invalidate name-dependent views. Sidebar rank lookups preserve
first duplicate ranks, stable missing entries, pins and folders. Album sorting
keeps each album in disc/track order and distinguishes same-named albums by ID
or URI. Descending album-name order does not reverse tracks inside an album.
Views still carry original source indices into playback and row actions.

No new request, credential/persistence schema, dependency, layout, typography or
navigation change. Existing queue protections and native themes are retained.
The independent promotion branch passed 950/974 library tests, binary/integration
suites, both Clippy configurations, documentation tests/Rustdoc, launcher tests,
Jekyll (4.362 seconds), and a 2m 24s release build/version smoke on Windows x64.
One existing native credential-store test remains ignored in ordinary suites.
Combined queue-branch verification is recorded in
[upstream-library-validation.md](../features/upstream-library-validation.md).
Protected cross-platform CI remains pending. No new CPU/RAM benchmark or manual
authenticated playback result is claimed. Upstream timing claims are upstream's.
