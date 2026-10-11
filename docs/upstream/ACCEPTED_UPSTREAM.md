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


## Power, session and tray scheduling

- `595ddc4e6ff4241f9c46e3b994ff48bd1cdd3c8a`, Dennis, Spotifast #728,
  port `82761fb`: platform readers and bounded pre-suspend/session-end saves.
- `fea496e4a2c51fa89899e3e07a39e6f9fd02114f`, Dennis, Spotifast #730,
  port `135a0df`: event-driven tray wait and Windows MMCSS audio task.
- `dd2d5e34e66d4e01e31e595a3022a1de808b449a`, Dennis, Spotifast #733,
  port `18a42e0`: hidden idle remote polling, with prompt command follow-up.

Cherry-pick conflicts contained context from the separately pending #732
minimized-close fix. That unrelated helper/test was excluded; the power and
scheduling changes were retained. No wholesale upstream merge was performed.
All fastframe crates, including the build dependency, now share maintainer fork
revision `f7dc6fbe938e8a953509d110cdba036f537e5b7e`. Its difference from the old
revision consists of documentation, MMCSS audio registration and condition-variable
headless waking. The MIT license and original copyright remain unchanged.
The upstream lockfile and Nix vendor hash travel with the dependency change.

Validation is in progress. Windows power GUIDs were checked against SDK
10.0.26100.0. Callback cancellation, saver policy, save acknowledgement, wake
races and playback polling are review boundaries. No energy/CPU improvement is
claimed without a new measurement. macOS reads Low Power Mode but this port does
not provide macOS sleep notifications. Linux watcher failure exits its watcher;
there is no reconnection promise. Flatpak portal/logind availability can differ.

SpotiPhile adaptation: Windows teardown checks every unregister result. If any
registration cannot be cancelled, its callback context and subscription parameters
are retained until process exit and a warning is logged, avoiding a potential
use-after-free. A hidden remote-playback regression preserves active polling.
Cancellation is performed outside callbacks and without holding their locks,
following [Microsoft's cancellation guidance](https://learn.microsoft.com/en-us/windows/win32/api/netioapi/nf-netioapi-cancelmibchangenotify2).
The [power unregister API](https://learn.microsoft.com/en-us/windows/win32/api/powersetting/nf-powersetting-powersettingunregisternotification)
reports success or failure; its public documentation does not explicitly describe
callback quiescence. Successful unregister is the upstream lifetime assumption;
real registration/drop is checked separately, not proof of all shutdown races.

Windows x64 validation completed for the source and teardown adaptation:
966 default and 990 all-feature library tests passed (two ignored in the full
runs), all binary/integration suites passed, both Clippy configurations passed
with warnings denied, formatting passed, and doctests/Rustdoc passed. The ignored
Windows desktop registration/drop test was explicitly run and passed separately.
All 21 tests in the pinned fastframe-shell dependency passed, including wake-before-
wait and cross-thread waking. Four launcher tests passed on the unchanged
LF-normalized WSL copy; Jekyll passed in 7.484 seconds. Release validation and
combined SpotiPhile queue validation are still pending at this checkpoint.

The optimized build completed in 2m 38s; `spotifast.exe --version` returned
`spotifast 0.12.0`. The lockfile and Nix vendor hash exactly match the reviewed
upstream snapshot, but the Nix build itself remains a CI gate.

Combined queue branch ports: 86a2d6e, 451cf50, 65ab8fb. Source hardening: a9b5f8c.
Combined validation passed: 1,028/1,052 library tests, binary/integration suites,
both Clippy configurations, formatting, doctests and warnings-denied Rustdoc.
Jekyll passed in 8.724 seconds. Release build passed in 2m 38s with version smoke.
See ../features/upstream-power-validation.md for explicit coverage limits.
