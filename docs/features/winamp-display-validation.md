# Winamp display restoration on macOS

The native Winamp mini player checks its saved position before creating its
macOS window. Its title-bar anchor must lie in the visible area of a connected
display, excluding the menu bar and Dock. A position on a disconnected display
is discarded so native window placement can choose the initial position.
AppKit screen coordinates are converted to window coordinates relative to the
top of the primary display. macOS positions remain in points, including on
Retina displays.

When AppKit cannot be queried off the main thread, or reports no screens, the
existing inclusive coordinate limits of -1000 through 5000 on each axis remain
the fallback. Nonfinite saved positions are rejected. Windows retains its
deferred restoration after the native scale is known; other platforms retain
their existing fixed-limit behavior.

## Source and adaptations

Ported from crmne/spotifast commit
[`54e222431566caeebc76e3cd6ef68b812ec2e0d9`](https://github.com/crmne/spotifast/commit/54e222431566caeebc76e3cd6ef68b812ec2e0d9),
“Open the macOS mini player on a connected display (#719) (#774)”.
Original author: HEOJUNFO <100739943+HEOJUNFO@users.noreply.github.com>.
Original co-authors: heojunfo <nationa@nation-a.com> and
Claude Opus 5.5 (1M context) <noreply@anthropic.com>.

The production port follows the cached upstream patch. The fixed-limit helper
is also compiled for tests on Windows so its fallback bounds can be covered
without AppKit. Added tests cover nonfinite positions and inclusive fallback
bounds, alongside both original upstream coordinate tests. No dependency,
shell, theme, frame pacing, persistence format or UI layout changes are needed.
The existing NSScreen dependency feature is already enabled.

## Validation status

Parent Windows validation passed 1,037 default and 1,061 all-feature library
tests (two existing ignored tests each), all target suites, formatting and both
warnings-denied Clippy configurations. Independent source review found no
blocker in coordinate conversion, platform guards, existing AppKit API features
or the fallback behavior. Doctests, warnings-denied Rustdoc, Jekyll, the optimized
build and `spotifast 0.12.0` smoke passed. This Windows environment cannot validate AppKit
behavior or physical macOS display disconnection.

On macOS hardware, verify these cases before claiming runtime coverage:

- Save the mini player on an external display, quit, disconnect that display,
  and restart directly into native Winamp mode. The player must be reachable.
- Keep the external display connected and verify its valid saved position is
  retained, including a display above or left of the primary display.
- Repeat with mixed Retina scaling, the Dock and menu bar on different
  displays, and shaded/unshaded Winamp windows.
- Verify returning to the main window preserves its independent geometry.

The pure coordinate tests cover disconnected-display rejection and AppKit's
vertical-coordinate conversion. They do not substitute for native window or
hardware validation. Off-main-thread and no-screen AppKit branches also remain
runtime-unverified; the fallback helper itself has focused regression coverage.
