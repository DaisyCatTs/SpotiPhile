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

## Decoder queue wakeups

Upstream: `277a4fabcd2ca11ced4adb45b274f29084fde7eb` (Spotifast #726).
Port commit: `18db688`, preserving Dennis's authorship, without adaptations.
Replaces 10 ms polling with a queue-consumption wakeup and a 50 ms fallback
wait. Queued audio remains bounded; seeking, transitions and visualizer signal
ownership are unchanged. Source changes are limited to src/sink.rs.

## Audio output recovery

Upstream: `13876a8e81c7abd90266a2c20d030c3e5958213b` (Spotifast #731).
Port commit: `87975a8`, preserving Dennis's authorship, without adaptations.
Depends on the queue-wakeup port. A failed output is maintained/reopened instead
of immediately stopping playback. A full queue with no progress for two seconds
reopens the stream; a suspension gap resets the stall timer. Reopening failures
still report an actionable playback error, and gain is reapplied after recovery.

Validation in progress on feature/upstream-audio-reliability. All 28 sink tests
passed on Windows, including wake thresholds/timeouts, stall timing, suspension,
existing pause/resume, output-opening failure, seek gating and gapless queue
coverage. Real Bluetooth/default-device switching is not yet manually verified.
Upstream's CPU measurements are not measurements of this SpotiPhile build.

Branch validation completed on Windows x64: full default/all-feature suites
passed (946/969 library tests, one existing credential-store test ignored),
integration suites passed, both Clippy configurations passed with warnings
 denied, and the optimized build passed in 2m 23s with a version smoke.
Protected cross-platform CI and actual device-switch testing remain pending.
