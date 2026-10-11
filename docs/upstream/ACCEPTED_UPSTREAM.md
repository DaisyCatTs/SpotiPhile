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

## Resampler, name-cache and ordering fixes

- Upstream `031e8c565c29bd72763bf29f0cfbe17c3446bc58`, Spotifast #681;
  port `577d24a`, Dennis. Portable eight-lane stereo reduction; other channel
  counts keep the scalar path. Existing phase/chunk logic is unchanged. Numerical
  comparison coverage is retained. No local throughput benchmark is claimed.
- Upstream `9dd23c9b27a0ad6d43d4569a36547f9806945451`, Spotifast #744;
  port `e985796`, Luís Fernando. Failed overlapping name lookups cannot overwrite
  a known name. Successful account renames still advance the name revision.
  No new API request, credential behavior or persistence schema.
- Upstream `9d74afe8ad5196eb67942f449d6ed57988d58697`, Spotifast #717;
  port `4be253a`, Лев Федорец. Cached rank lookups replace repeated sidebar order
  scans. First duplicate ranks, stable missing entries, pins and folders are
  retained. No spacing, typography, shell or navigation redesign.
- Upstream `5b7a5dfe8c15542ee82fd7560c1e36496306b6c9`, Spotifast #701;
  port `8240528`, Samet Karaş. Album sorting groups by identity and disc/track
  order, including URI fallback when IDs are empty. Descending reverses album
  names while keeping songs inside each album in track order. Original item
  indices remain attached to rows, preserving playback/action mapping.

All four ports applied without source adaptations and preserve original author
and co-author metadata. Branch-specific checks are in progress. Platform CI,
new performance measurements and live authenticated behavior are not yet claimed.
