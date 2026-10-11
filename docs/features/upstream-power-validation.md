# Power and hidden-work validation

## Scope

SpotiPhile ports Spotifast #728, #730 and #733 while retaining queue controls,
Winamp, native themes and MilkDrop. Original SHAs and port commits are in
[the accepted ledger](../upstream/ACCEPTED_UPSTREAM.md). All fastframe crates,
including build-time i18n, share f7dc6fbe938e8a953509d110cdba036f537e5b7e.
The dependency's diff against 23d87e0 was reviewed: shell condition-variable
waking, Windows MMCSS registration and documentation. MIT attribution is retained.
Cargo.lock and flake.nix match the reviewed upstream snapshot exactly.

Windows notification cancellation now checks failures and retains callback
allocations on failure instead of freeing potentially live context. Hidden remote
playback retains active polling. The separately pending minimized-close port was
excluded from cherry-pick conflict context. Queue module registration was retained.

## Focused promotion branch

Full default/all-feature suites passed: 966/990 library tests, two ignored in
ordinary runs, and all binary/integration suites. Both Clippy configurations,
formatting, doctests and warnings-denied Rustdoc passed. The Windows desktop
registration/drop test passed when explicitly invoked. All 21 pinned shell tests
passed, including pre-existing wake and cross-thread wake behavior. Four unchanged
launcher tests passed in the established LF-normalized WSL copy. Jekyll passed
in 7.484 seconds. Optimized build passed in 2m 38s and the version smoke passed.

PR #4 is stacked on #3, #2 and #1. Main promotion still requires protected CI
and approving review. No protection bypass or wholesale upstream merge.

## Combined queue branch

Default/all-feature suites passed: 1,028/1,052 library tests, two ignored in each
ordinary run. Binary/integration suites passed, including queue controls.
Both Clippy configurations passed with warnings denied, as did doctests and
all-features Rustdoc with warnings denied. Combined Jekyll passed in 8.724 seconds.
The optimized build passed in 2m 38s; the version smoke returned
spotifast 0.12.0. Formatting and final batch diff checks passed.

## Coverage limits

Tests ran on Windows x64. WSL launcher/docs checks are not Linux application
runtime validation. Cross-platform CI and Nix packaging remain separate gates.
The desktop smoke registers, reads and drops watchers; it does not exercise an
actual suspend, logout, battery transition, tray click or audio glitch under QoS.
No new authenticated playback or CPU/RAM benchmark is claimed.

macOS reads Low Power Mode with an availability guard and retains AppKit idle;
this bundle does not implement macOS sleep notifications. Linux watcher failures
exit their threads without reconnecting, and logind/portal access can vary inside
Flatpak. Sleep acknowledgement waits are bounded, but disk persistence success
is not guaranteed if storage fails. Existing persistence paths retain error logs
and dirty/retry semantics. Windows successful unregister retains upstream's
callback-lifetime assumption; cancellation failure now deliberately leaks a small
allocation until process exit for safety. No credentials or settings are migrated.
