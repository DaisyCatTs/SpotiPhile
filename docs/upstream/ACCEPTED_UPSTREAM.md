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

Windows x64 branch validation completed: 950/974 library tests passed with one
existing credential-store test ignored, all binary/integration suites passed,
both Clippy configurations passed with warnings denied, and formatting,
documentation tests and Rustdoc passed. Four launcher tests passed in WSL using
the established LF-normalized packaging copy. Jekyll passed in 4.362 seconds.
The optimized build passed in 2m 24s and its version smoke passed. Cross-platform
CI and performance measurements remain separate; no manual hardware claim.

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


## Fresh stable promotions

Library PR #6 merged to main at `38c29f2` after protected CI passed.
Power ports are replayed on that main without the queue or appearance redesign.

| Upstream | Fresh promotion |
| --- | --- |
| `595ddc4e6ff4241f9c46e3b994ff48bd1cdd3c8a` | `24881d1` |
| `fea496e4a2c51fa89899e3e07a39e6f9fd02114f` | `acf2a4a` |
| `dd2d5e34e66d4e01e31e595a3022a1de808b449a` | `99e28f5` |

Windows teardown hardening and hidden active-playback regression: `c373d50`.
Only conflict was module context: register power, omit the absent queue-edit
module. Protected CI must validate this fresh stable candidate. Prior checks
above describe the original focused and combined branches, not this new SHA.
The owner authorized zero required approving reviews; all seven strict required
CI contexts, administrator enforcement and linear history remain enabled.

## Remaining functional ports, 2026-10-11

This candidate is based on main `d6be3e8`, after power/tray #7 and ordered queue
playlist saving #9 merged through protected checks. It deliberately ports six
remaining changes. Together with main's ten upstream ports, the candidate
contains sixteen of the seventeen commits after the immutable baseline.

That count refers to the previously reviewed `dd2d5e3` snapshot. A subsequent
fetch found twenty newer commits; they remain individually reviewed rather than
being merged wholesale. One additional security port is recorded below.

| Original upstream SHA | Candidate commit | Author | Behavior |
| --- | --- | --- | --- |
| `17f3a0db3547aee20b24c432a7b8fb5fe1709592` | `6ca455b` | Dennis | Closing a minimized window follows the existing tray setting from the logic pass. |
| `12deee472ee87e2c9ed0e0f13730cbdd9a6cfa45` | `e456a7f` | Dennis | Automatic Mac GPU switching and MilkDrop surface update after monitor moves. |
| `eff77cd340dc5312a2f76b59c7a2ee73d0011867` | `d841503` | Ali Kaan | All 25 upstream Turkish wording corrections. |
| `88d5831f0d63e797579b4cdccfdc94d19efd85cb` | `449869b` | Volodymyr Bunchuk | Ukrainian locale, catalogue and diagnostics. |
| `d692a2276e11a597b579ef033e09f0c14dfbe018` | `898c4dc` | Göktuğ Ekinci | Active collection shuffle retains the current theme's hover accent. |
| `cbe817610546c8d1b023ab3977dd89ede65d372e` | `0cc79e3` | Luís Fernando | Contributor attribution for an existing Spotify-owned Blend. |

Original author/co-author metadata is retained. The close port uses our previously
validated adaptation, retaining both the power handler and tray timing regression.
Blend recognition uses the loaded adders: the listener plus another person.
This is an inference from metadata, not a new Spotify Blend API or creation flow.
No collaboration/editing claim is added. Editorial mixes remain unchanged.

Additional tests and the offline Blend review fixture are in `66b4ae2`. Hover is
tested with dark, light and custom blue palettes, not a hardcoded green. Blend
rendering checks narrow/wide responsive columns, the byline and read-only label.
See [validation](../features/upstream-final-functional-validation.md).

## Prevent accidental token debug logging

Upstream `e9854af0934affcb29e29b2f442b0b922a953772`, Douglas Silva, Spotifast #756,
port `c0edf51`. StoredToken no longer implements Debug, preventing ordinary debug
formatting from printing usable access/refresh tokens. Serialization, protected
storage, cloning and equality stay unchanged. Existing persistence assertions
still compare the complete value without debug-printing it on failure.

Positive construction and negative debug-formatting doctests guard the contract.
The security revision passed 974/998 default/all-feature library tests, both
Clippy configurations, formatting, doctests and warnings-denied Rustdoc. Final
optimized build passed in 3m 19s. Seventeen of the thirty-seven commits in the
newly fetched snapshot are now deliberately ported on this candidate.

## Reserve the Web API callback before sign-in

Original `98b37e7d333c76193f80967ab301b089de010c60`, Sam Boland, Spotifast #768.
Focused port `f9fc182` retains original author credit and the source SHA in its
commit message. Adaptation `54adcd5` isolates the backend regression from the
machine's fixed callback port, checks both shared/personal errors and verifies
existing credential generations and restore flags survive a bind failure.

The listener is acquired before grant invalidation, browser launch or new
authorization state. Production callback identities, ports, PKCE/state checks,
protected storage and independent playback consent stay unchanged. No new
permission or network destination is added. See
[validation and limitations](../features/login-port-validation.md).

## Isolate released-port callback regressions

Original `f91ee83166b7b4a0474d48a7dd25f39231ed8759`, Dennis, Spotifast #847,
focused port `a48d5ab`. The released-listener test retries setup on a fresh port
when parallel tests claim its ephemeral port. Our added Unix completed-callback
test uses the same principle for an entire three-rebind sequence, retaining
failure for every other error and for ten unsuccessful attempts. Production
listener settings remain unchanged. The original Nix failure and pending fresh
Unix validation are recorded in the login feature validation document.

## Bound external inputs and preset filenames

Three focused ports retain Carmine Paolino's original author credit:

| Original upstream SHA | Port | Behavior |
| --- | --- | --- |
| `34aa7dffbc25c4477abadeddeb9c17420a241bd0` | `8521f5e` | Reject non-plain MilkDrop preset pack filenames before joining the destination. |
| `f70a58acb8bf5adee89463884c5a750e61a1b12c` | `486192f` | Bound incoming artwork as received; reject inconsistent STORED ZIP sizes. |
| `b60ea0120793a6b10edfef5ba69e6d5f1ab9c3f1` | `9ce7e69` | Bound each callback request line by bytes and time. |

The callback port's adjacent-test conflict was resolved by retaining both
regressions and our existing listener/configuration fixes. All local Windows
gates and independent source review passed. Native macOS/Linux execution and
protected promotion remain outstanding. See
[validation and precise limits](../features/input-hardening-validation.md).

## Rustls TLS encryption-level boundary patch

Original `68c453e1c9fc524bca8d13f12a4ae6cbd9ad6465`, Carmine Paolino,
coauthor Chris (`25006584+seichris@users.noreply.github.com`). Adaptation
`b2e8796` updates only Rustls 0.23.43 to 0.23.45 and its registry checksum,
retaining the existing tempfile/getrandom resolution. Official advisory
RUSTSEC-2026-0285 identifies the fixed version. No transport API, account
permission or credential format changes.

Windows tests, lint, docs and optimized build passed; independent review found
no material findings. This candidate is not yet promoted. Its fork-specific
Nix vendor hash and full platform gates remain pending. See
[validation](../features/rustls-patch-validation.md).
