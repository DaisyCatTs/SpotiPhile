# Upstream review, 2026-10-11

Renamed fork: `DaisyCatTs/SpotiPhile`. Local origin now uses its new URL.
Upstream remains `crmne/spotifast` with the inert push guard. Safe fetches used
`--no-tags`. At the initial fetch, no upstream commit was integrated. Deliberate ports followed as recorded below.

Immutable baseline: `995c768dcba4da7f93302bf2107af6b7ec3df02b`.
Fetched upstream: `dd2d5e34e66d4e01e31e595a3022a1de808b449a`.
Origin main: 0 ahead / 17 behind. At the initial fetch, local HEAD: 8 ahead / 17 behind
(uncommitted feature work excluded).

## Pending individual reviews

| Commit | Change | Status |
| --- | --- | --- |
| [12deee4](https://github.com/crmne/spotifast/commit/12deee472ee87e2c9ed0e0f13730cbdd9a6cfa45) | Let dual-GPU Macs keep the integrated GPU for Spotifast (#736) | Accepted: `6d25541` |
| [eff77cd](https://github.com/crmne/spotifast/commit/eff77cd340dc5312a2f76b59c7a2ee73d0011867) | fix(i18n): refine Turkish translations for consistency and natural phrasing (#668) | Accepted: `bfacfc2` |
| [031e8c5](https://github.com/crmne/spotifast/commit/031e8c565c29bd72763bf29f0cfbe17c3446bc58) | Let the stereo resampler use vector registers on every target (#681) | Accepted: `2229f97` |
| [9d74afe](https://github.com/crmne/spotifast/commit/9d74afe8ad5196eb67942f449d6ed57988d58697) | Reduce library sorting work during scrolling (#717) | Accepted: `f420bb7` |
| [88d5831](https://github.com/crmne/spotifast/commit/88d5831f0d63e797579b4cdccfdc94d19efd85cb) | feat(i18n): add Ukrainian language support (#718) | Accepted: `6020ff5` |
| [277a4fa](https://github.com/crmne/spotifast/commit/277a4fabcd2ca11ced4adb45b274f29084fde7eb) | Wait for room in the audio queue instead of polling it (#726) | Accepted: `52f29af` |
| [17f3a0d](https://github.com/crmne/spotifast/commit/17f3a0db3547aee20b24c432a7b8fb5fe1709592) | Hide a minimised window to the tray when it is closed (#725) | Accepted: `3c2f103` |
| [f5a946c](https://github.com/crmne/spotifast/commit/f5a946c6bb1b332a1147153821172cf08eb84d30) | Keep the resume point and history safe when a session ends abruptly (#727) | Accepted: `4c62229` |
| [9dd23c9](https://github.com/crmne/spotifast/commit/9dd23c9b27a0ad6d43d4569a36547f9806945451) | Keep an adder's name once it is found (#744) | Accepted: `6cad104` |
| [d692a22](https://github.com/crmne/spotifast/commit/d692a2276e11a597b579ef033e09f0c14dfbe018) | Keep the playlist shuffle button green while hovered (#722) | Pending |
| [cbe8176](https://github.com/crmne/spotifast/commit/cbe817610546c8d1b023ab3977dd89ede65d372e) | Show Added By in Blends (#745) | Pending |
| [5b7a5df](https://github.com/crmne/spotifast/commit/5b7a5dfe8c15542ee82fd7560c1e36496306b6c9) | Keep each album's songs in track order when sorting by album (#701) | Accepted: `8ea6ffa` |
| [14db97c](https://github.com/crmne/spotifast/commit/14db97c82263400d180100135d631c9885245ce2) | Add Linux custom title bar (#648) | Pending |
| [595ddc4](https://github.com/crmne/spotifast/commit/595ddc4e6ff4241f9c46e3b994ff48bd1cdd3c8a) | Follow the system's power and session state (#728) | Accepted: `86a2d6e` |
| [fea496e](https://github.com/crmne/spotifast/commit/fea496e4a2c51fa89899e3e07a39e6f9fd02114f) | Sleep in the tray until something is due (#730) | Accepted: `451cf50` |
| [13876a8](https://github.com/crmne/spotifast/commit/13876a8e81c7abd90266a2c20d030c3e5958213b) | Keep playing when the audio output changes (#731) | Accepted: `ed30344` |
| [dd2d5e3](https://github.com/crmne/spotifast/commit/dd2d5e34e66d4e01e31e595a3022a1de808b449a) | Poll other devices less often when nobody can see the app (#733) | Accepted: `65ab8fb` |

Prioritize abrupt-session history/resume safety (`f5a946c`), audio-output
recovery (`13876a8`), sink backpressure (`277a4fa`), then polling/tray/power
improvements (`dd2d5e3`, `fea496e`, `595ddc4`). Review platform and dependency
assumptions before porting. Port functional fixes separately from UI changes.

None is accepted merely because it appears here. Each deliberate port needs
the original SHA, adapted behavior, our commit and executed checks in
ACCEPTED_UPSTREAM. No blind merge into stable main.

## Keeping Spotiphile current

The read-only upstream-watch workflow runs on relevant dev/feature pushes and
supports manual dispatch. Its daily 08:17 UTC schedule becomes active only when
the workflow exists on GitHub's default branch (currently main). This work does
not change that default branch or merge into stable main. Reports distinguish
accepted, rejected and pending original SHAs from the ledgers.

A GitHub behind badge measures ancestry, not whether a fix was ported. Focused
ports retain our linear development history and can still leave that badge at
17 behind. The accepted ledger is the evidence of functional integration. The
remaining three commits still need individual review and validation.


The owner subsequently authorized validated promotion to main. Protected PR #1
contains persistence safety and the read-only watcher. PR #2 contains the two
audio ports and is stacked on #1. Required CI and GitHub approval remain gates.
The paused appearance redesign is excluded from both promotion PRs.

## Next port boundaries

- Resampler #681: single-module portable stereo reduction and a numerical
  regression. Review rounding and whole/chunk equivalence; do not describe
  upstream's Arm benchmark as a measurement of our build.
- Name-cache #744: failed overlapping lookups must not replace a known name.
  Preserve revision invalidation and successful rename behavior.
- Sidebar ordering #717 and album ordering #701: functional ordering fixes,
  separately tested against Daisy queue selections and row identity. These do
  not authorize shell, spacing, typography or appearance redesign.
- Power #728, tray scheduling #730 and hidden polling #733 form a dependency
  bundle. #730 moves every fastframe crate together to pinned maintainer fork
  revision f7dc6fbe938e8a953509d110cdba036f537e5b7e and refreshes the Nix hash.
  Review callback teardown, bounded save acknowledgments, Windows thread QoS,
  platform availability and real tray/media-key wakeups before accepting it.
  MilkDrop must retain the short headless tick. macOS keeps its AppKit tray loop;
  its power reader does not implement sleep/wake notifications. Linux's optional
  logind delay inhibitor can be unavailable in a sandbox and must remain optional.

Windows power/thread policy should be checked against Microsoft's
[Quality of Service](https://learn.microsoft.com/en-us/windows/win32/procthread/quality-of-service)
and [MMCSS](https://learn.microsoft.com/en-us/windows/win32/procthread/multimedia-class-scheduler-service)
documentation. These are review inputs, not evidence of a completed power port.


PR #3 adds the reviewed resampler, name-cache, sidebar-ordering and album-ordering
ports, stacked on #2. These four are now ported and separately validated on the
promotion branch; combined queue-feature verification is a separate record.
The power/tray/polling bundle was subsequently ported as recorded below.
The remaining platform, language and visual changes are pending.


PR #4 stacks the power/tray/polling bundle on #3. Windows focused-branch
checks passed, including the release build and pinned shell wakeup tests.
Combined queue validation passed, including both full suites, Clippy, docs
and the optimized build. GitHub approval and protected
platform checks remain required before stable promotion.

Stable promotion: #1 and #5 merged into main. #6 queues library/resampler
ports against main with auto-merge. #3/#4 feature-base merges are not main
promotion. Solo-maintainer review count is zero; strict required CI and
linear-history protections remain. Minimized-close Windows combined checks and optimized build passed, as
recorded in docs/features/upstream-close-validation.md.

Mac GPU and language ports passed Windows combined checks and release build.
Ukrainian coverage includes 25 fork strings; Turkish retains all 25 upstream
wording corrections. Hardware GPU switching and fluent review remain separate.
Three original commits remain pending, as listed in the table.
