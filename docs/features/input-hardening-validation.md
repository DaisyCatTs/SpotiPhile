# Bounds on external inputs

This focused candidate preserves native playback, MilkDrop and Winamp while
rejecting malformed or oversized inputs at existing boundaries. It introduces
no dependency, permission, setting, persistence schema or interface redesign.

## Supplier provenance

All three original commits are authored by Carmine Paolino. Original author
metadata and source SHAs are retained in the port commits.

| Original upstream commit | Port | Boundary |
| --- | --- | --- |
| `34aa7dffbc25c4477abadeddeb9c17420a241bd0` | `8521f5e` | MilkDrop pack filenames |
| `f70a58acb8bf5adee89463884c5a750e61a1b12c` | `486192f` | Artwork responses and STORED ZIP entries |
| `b60ea0120793a6b10edfef5ba69e6d5f1ab9c3f1` | `9ce7e69` | OAuth callback request lines |

Preset extraction still flattens pack directories. Only one ordinary filename
component without a colon may be joined to the preset directory. A crafted pack
test rejects a Windows drive-relative name while retaining a valid preset.
This filename check does not claim symlink-safe extraction into a locally
tampered destination directory.

Incoming artwork checks declared size and accumulated response chunks against
the existing 8 MiB limit before retaining additional bytes. Regressions cover
oversized bodies both with and without Content-Length. This change does not
bound arbitrary pre-existing files placed into the local cache by other means.

A STORED ZIP entry must have equal compressed and declared sizes before its
bytes are copied. A forged directory-size regression checks that the declared
per-entry limit cannot be bypassed by a larger stored payload.

Each accepted OAuth callback connection gets five seconds and at most 8 KiB for
its request line. A real-socket regression queues a stalled request, an oversized
line and a valid synthetic callback. The valid callback still succeeds. The
listener remains serial; the bounds do not eliminate every local denial-of-service
possibility. The conflict during porting was only between adjacent tests: both
the existing Unix rebind regression and the new bounded-read regression remain.
PR #13's socket settings, personal grant and accepted Client ID fixes are retained.

## Validation

Independent source review found no blockers. Windows validation passed 1,036
default and 1,060 all-feature library tests (two existing ignored each), all
target suites, formatting, both warnings-denied Clippy configurations, doctests,
warnings-denied Rustdoc, Jekyll, optimized build and `spotifast 0.12.0` smoke.
No real Spotify credentials or authenticated callbacks are used.
Native macOS/Linux execution requires protected CI before promotion.

Promotion branch is based on main after PR #13 merged (`1e75f2c`). Its source,
Cargo manifests, lockfile and Nix configuration match the locally validated
candidate exactly. Promotion ports are `799293a`, `814ae58` and `e8f8fcc`,
respectively; original candidate SHAs above identify the validation checkpoint.
The PR must pass its own required checks before squash promotion.
