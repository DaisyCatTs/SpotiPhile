# Remaining functional upstream ports

User-visible changes are contributor attribution for existing Blends, active
shuffle hover feedback, Ukrainian/Turkish language support, minimized-close tray
behavior and Mac GPU switching. No shell, navigation, density or theme redesign.

Full source mapping and retained author credit are in
[the accepted ledger](../upstream/ACCEPTED_UPSTREAM.md). The Linux custom title bar
remains deferred for separate visual review. The immutable supplier baseline is
unchanged; GitHub's behind badge still measures ancestry rather than ported fixes.

## Functional checks

The focused localization integration test passed. A separate catalogue check
verified all 25 upstream Turkish wording changes and no empty Ukrainian singular
translations. Hover regression passed for dark, light and a custom blue accent.
Rendered Blend regression passed for dark/light and 1100/1600-pixel widths: the
byline appears on both; Added By follows the existing extra-wide column policy;
the playlist is never labelled Collaborative Playlist.

The initial rendered regression assumed Added By was visible at 1100 pixels.
Inspection showed the existing table hides that secondary column below 920
content pixels. The corrected test verifies narrow byline attribution and wide
column attribution, preserving the existing layout contract.

Windows x64 full suites passed: 974 default and 998 all-feature library tests
(two existing ignored tests), plus all binary/integration suites. Both Clippy
configurations, formatting, doctests and warnings-denied Rustdoc passed. The
optimized build passed in 2m 39s and its version smoke returned spotifast 0.12.0.
Four unchanged launcher tests passed in the established LF-normalized WSL copy.
Jekyll passed in 5.362 seconds. Mac plist parsing confirmed the automatic graphics
switching opt-in. Protected remote platform CI remains a separate gate.

## Visual evidence

Actual baseline captures use main `d6be3e8` plus the identical offline Blend demo
fixture (`75eca82`). Candidate fixture/tests are `66b4ae2`. Data, window dimensions
and themes match. The fixture suppresses playlist artwork to avoid timing-dependent
cover differences. It does not sign into Spotify or change an account.
The [HTML comparison](../reviews/upstream-remaining/index.html) provides matching
1100x800 and 1600x800 light/dark captures, original-image links and Before/After
controls. Candidate captures were inspected: narrow byline and wide Added By
appear without overlap, while the native layout remains intact.
All eight viewer states loaded successfully in the collaborative browser, with
matching natural image dimensions and active Before/After button states. The
browser snapshot tool failed; direct browser DOM/image-load checks succeeded.

One initial long-delay baseline capture landed on Home instead of the requested
playlist and displayed a duplicate widget-ID diagnostic. It was invalid for
comparison and was retaken with the established 500 ms delay. This records the
observation; the bundle does not claim to fix that unrelated demo/navigation issue.

## Coverage limits

Windows x64 is the local application/test platform. WSL documentation/launcher
checks do not prove Linux runtime behavior. Mac GPU switching is compiled through
protected platform CI, not tested on dual-GPU Mac hardware here. No new CPU/RAM,
energy, authenticated Blend or sleep/logout measurement is claimed. The Blend
classification is the upstream inference from supplied adders, not an explicit
Spotify capability flag. No new request, grant or storage migration is introduced.

## Additional security and inventory checks

A subsequent safe fetch found twenty newer upstream commits, including
StoredToken Debug hardening. That small security fix was deliberately ported:
token values cannot be debug-formatted, while their protected storage format
remains compatible. Positive/compile-fail doctests passed. The final security
revision again passed 974/998 library tests, binary/integration checks, both
Clippy configurations, formatting, doctests and Rustdoc. Its optimized build
passed in 3m 19s.

The read-only upstream watcher now distinguishes explicit Deferred and Rejected
ledger entries. A mentioned SHA alone is not treated as rejection. Two Python
regressions cover both statuses, incidental mentions and conflicting decisions;
they are included in required quality CI. New upstream dependency and networking
changes remain pending individual review, not silently accepted.
