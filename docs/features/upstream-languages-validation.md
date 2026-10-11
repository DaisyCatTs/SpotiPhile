# Mac GPU and translation validation

Deliberate ports: 12deee4 (#736), eff77cd (#668), 88d5831 (#718).
Original full SHAs/authors and port SHAs are recorded in the accepted ledger.
SpotiPhile catalogue adaptation is 0126d08. It adds 25 Ukrainian translations
for fork-specific controls/branding and refreshes all catalogue references.
No dependency, queue state contract, credential or persistence change.

The Mac bundle enables automatic graphics switching; MilkDrop follows monitor
changes by resizing/updating the CGL context, with zero-size guards. Installed
glutin 0.32.3 code confirms resize calls context.update() and supports offline
renderers. Apple QA1734/TN2229 were reviewed. Windows plist parsing passed;
macOS compilation/bundle CI and dual-GPU hardware tests remain separate gates.
No hardware, battery-life or GPU performance improvement is measured here.

## Executed checks

Windows x64 full default/all-feature suites passed (1,029/1,053 library tests,
two ignored in ordinary runs). All binary/integration tests, including the
unchanged catalogue completeness/placeholder contract, passed. Both Clippy
configurations passed with warnings denied. Formatting, all-feature doctests,
warnings-denied Rustdoc and gettext regeneration --check passed. A before/after
comparison confirmed all 25 upstream Turkish wording corrections remain.
Active Ukrainian keys match the template; no empty singular translations remain.
Plural, date and system-locale tests are retained. The 25 additional translations
have no independent fluent-speaker review; structural coverage is not that review.

Four unchanged launcher tests passed on the established LF-normalized WSL copy.
Jekyll passed in 9.425 seconds. Native demo screenshots were inspected in Ukrainian
at 1280x800 dark and 900x650 light. Playlist, queue and player labels fitted the
inspected states without visible overflow. Demo shots are isolated from Spotify
sign-in and do not demonstrate authenticated playback or macOS behavior.

Initial full test attempts failed the catalogue-count assertion (677 vs 678).
Retired Ukrainian branding entries caused the extra parsed entry. Removing the
obsolete entries fixed the original test; assertions and completeness requirements
were not weakened. The initially missing multiline branded sign-in string was
translated as well. Final suites above include these fixes.

The optimized build passed in 2m 40s; version smoke returned spotifast 0.12.0. Cross-platform CI
and stable promotion are separate requirements. No UI redesign was performed.
