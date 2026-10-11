# Minimized close-to-tray validation

Upstream 17f3a0db3547aee20b24c432a7b8fb5fe1709592 (Spotifast #725),
Dennis; SpotiPhile port 3c2f103. Close intent is evaluated in the logic pass
instead of painting, preserving background playback when the window is minimized
and Keep playing in the background is enabled with an available tray.
Explicit Quit, no-tray and window-switch conditions retain their guards.

Conflict resolution retained the power handler and the event-driven tray test.
The final source diff contains only the upstream behavior and regression test.
No new layout, theme, auth, persistence format or dependency change.

Windows x64 checks passed: full default/all-feature suites (1,029/1,053 library
tests, two ignored in each ordinary run), all binary/integration tests, both
Clippy configurations with warnings denied, formatting, all-feature doctests and
warnings-denied Rustdoc. Four unchanged launcher tests passed on the established
LF-normalized WSL copy. Jekyll passed in 7.897 seconds.
Optimized build passed in 2m 38s; version smoke returned spotifast 0.12.0.
The final batch diff check passed.

No manual minimized-window/tray interaction or macOS/Linux runtime test is
claimed. Cross-platform CI and stable promotion remain separate gates.
Queue undo/redo and batch controls were included in the combined test suites.
