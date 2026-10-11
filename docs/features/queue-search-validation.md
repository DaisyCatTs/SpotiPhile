# Queue search validation

Search is available on the queue page and in the Queue inspector tab. It uses
the existing native search-field widget and translated Search/No matches labels.
The Recents tab is unchanged. No network request, dependency, credential or
persistence-format change is introduced.

## Contract

Case-insensitive substring search covers titles, artists, albums and podcast
shows in loaded upcoming rows. It preserves original occurrence indexes and
section ownership. Current playback stays visible. Search never replaces the
playback context or modifies playback order.

Selection resets on a query change. Select All and Shift-selection include
visible occurrences only. Newly hidden rows lose selection after a metadata
update. Existing snapshot guards continue protecting edits against playback
advancement. Clear queue, positional drops and keyboard moves are unavailable
while filtering. Explicit top/bottom commands use the whole manual queue.

Results are cached between unchanged frames, with invalidation for metadata,
query and queue changes. An empty query caches indices only; clearing search
releases the cached metadata allocation. Rows remain virtualized. This is not a new measured
CPU/RAM baseline. Search covers loaded rows only and makes no request for unseen
context pages. Query/cache are ephemeral and reset with account data.

## Verification

Final Windows x64 default/all-feature suites passed: 1,035/1,059 library tests,
two existing ignored tests in each ordinary run, and all binary/integration
suites. Both Clippy configurations, formatting, doctests and warnings-denied
Rustdoc passed. Four unchanged launcher tests passed using the established
LF-normalized WSL copy. Jekyll built the documentation in 8.235 seconds.

Regressions cover metadata updates, track and episode matching, duplicates,
12,000 rows, original-index selection/removal, cache reuse/release, native
rendered matches and empty results. Native demo captures were inspected at
1280x800 dark (queue inspector) and 900x650 light (queue page). The search field
fits without overlap; loaded queue and player controls remain visible.

An initial test fixture used Artist instead of ArtistRef and was corrected.
The first optimized compile was intentionally interrupted to correct metadata
retention with an empty query. The final optimized build and `spotifast 0.12.0`
version smoke passed after that correction. Remote platform CI is dispatched
separately and is not claimed as completed here.
No new authenticated playback or Linux/macOS application runtime is claimed.
