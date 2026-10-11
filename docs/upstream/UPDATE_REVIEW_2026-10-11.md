# Upstream review, 2026-10-11

Renamed fork: `DaisyCatTs/SpotiPhile`. Local origin now uses its new URL.
Upstream remains `crmne/spotifast` with the inert push guard. Safe fetches used
`--no-tags`. No upstream commit was merged, cherry-picked or rebased.

Immutable baseline: `995c768dcba4da7f93302bf2107af6b7ec3df02b`.
Fetched upstream: `dd2d5e34e66d4e01e31e595a3022a1de808b449a`.
Origin main: 0 ahead / 17 behind. At the initial fetch, local HEAD: 8 ahead / 17 behind
(uncommitted feature work excluded).

## Pending individual reviews

| Commit | Change | Status |
| --- | --- | --- |
| [12deee4](https://github.com/crmne/spotifast/commit/12deee472ee87e2c9ed0e0f13730cbdd9a6cfa45) | Let dual-GPU Macs keep the integrated GPU for Spotifast (#736) | Pending |
| [eff77cd](https://github.com/crmne/spotifast/commit/eff77cd340dc5312a2f76b59c7a2ee73d0011867) | fix(i18n): refine Turkish translations for consistency and natural phrasing (#668) | Pending |
| [031e8c5](https://github.com/crmne/spotifast/commit/031e8c565c29bd72763bf29f0cfbe17c3446bc58) | Let the stereo resampler use vector registers on every target (#681) | Pending |
| [9d74afe](https://github.com/crmne/spotifast/commit/9d74afe8ad5196eb67942f449d6ed57988d58697) | Reduce library sorting work during scrolling (#717) | Pending |
| [88d5831](https://github.com/crmne/spotifast/commit/88d5831f0d63e797579b4cdccfdc94d19efd85cb) | feat(i18n): add Ukrainian language support (#718) | Pending |
| [277a4fa](https://github.com/crmne/spotifast/commit/277a4fabcd2ca11ced4adb45b274f29084fde7eb) | Wait for room in the audio queue instead of polling it (#726) | Pending |
| [17f3a0d](https://github.com/crmne/spotifast/commit/17f3a0db3547aee20b24c432a7b8fb5fe1709592) | Hide a minimised window to the tray when it is closed (#725) | Pending |
| [f5a946c](https://github.com/crmne/spotifast/commit/f5a946c6bb1b332a1147153821172cf08eb84d30) | Keep the resume point and history safe when a session ends abruptly (#727) | Accepted: `4c62229` |
| [9dd23c9](https://github.com/crmne/spotifast/commit/9dd23c9b27a0ad6d43d4569a36547f9806945451) | Keep an adder's name once it is found (#744) | Pending |
| [d692a22](https://github.com/crmne/spotifast/commit/d692a2276e11a597b579ef033e09f0c14dfbe018) | Keep the playlist shuffle button green while hovered (#722) | Pending |
| [cbe8176](https://github.com/crmne/spotifast/commit/cbe817610546c8d1b023ab3977dd89ede65d372e) | Show Added By in Blends (#745) | Pending |
| [5b7a5df](https://github.com/crmne/spotifast/commit/5b7a5dfe8c15542ee82fd7560c1e36496306b6c9) | Keep each album's songs in track order when sorting by album (#701) | Pending |
| [14db97c](https://github.com/crmne/spotifast/commit/14db97c82263400d180100135d631c9885245ce2) | Add Linux custom title bar (#648) | Pending |
| [595ddc4](https://github.com/crmne/spotifast/commit/595ddc4e6ff4241f9c46e3b994ff48bd1cdd3c8a) | Follow the system's power and session state (#728) | Pending |
| [fea496e](https://github.com/crmne/spotifast/commit/fea496e4a2c51fa89899e3e07a39e6f9fd02114f) | Sleep in the tray until something is due (#730) | Pending |
| [13876a8](https://github.com/crmne/spotifast/commit/13876a8e81c7abd90266a2c20d030c3e5958213b) | Keep playing when the audio output changes (#731) | Pending |
| [dd2d5e3](https://github.com/crmne/spotifast/commit/dd2d5e34e66d4e01e31e595a3022a1de808b449a) | Poll other devices less often when nobody can see the app (#733) | Pending |

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
remaining 16 commits still need individual review and validation.
