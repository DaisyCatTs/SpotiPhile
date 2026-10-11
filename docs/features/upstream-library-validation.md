# Upstream library/resampler fixes with SpotiPhile queue controls

Windows x64 validation, 2026-10-11, feature/queue-controls. Original upstream
and promotion SHAs are recorded in ../upstream/ACCEPTED_UPSTREAM.md. Queue-feature
ports are 2229f97, 6cad104, f420bb7 and 8ea6ffa; authors/co-authors are preserved.

## Behavior and review

- Stereo resampling uses portable split sums; scalar other-channel behavior,
  filter phases and chunk processing remain. Numerical and chunk equivalence
  regressions pass. No throughput or quality improvement measurement is claimed.
- Failed overlapping adder-name lookups cannot overwrite a known name. Successful
  renames still advance the revision and invalidate name-dependent view caches.
- Sidebar rank lookups preserve first duplicate ranks, stable missing entries,
  pins, folders and local arrangement. No shell or layout change.
- Album sorting groups distinct identities, including URI fallback for empty IDs,
  and keeps disc/track order in both ascending/descending album-name sorts.
- Sorted views retain source row indices and construct playback URIs from those
  indices. Existing native queue selection, row editing and undo/redo suites pass.

No credential, network-request, persistence-schema or dependency change. No UI
redesign, theme migration, Pure Shuffle or audio-capability expansion.

## Executed checks

Formatting passed. Full default/all-feature suites passed: 1,012/1,036 library
tests, each with one existing native credential-store test ignored. Binary and
integration suites passed. Both Clippy configurations passed with warnings denied.
The optimized build passed in 2m 37s and the version smoke passed. Documentation tests and all-features Rustdoc with warnings denied passed. The combined Jekyll build passed in 8.489 seconds; the upstream inventory reported seven accepted source commits and ten pending.

Independent promotion branch validation passed 950/974-library-test suites,
Clippy, docs tests/Rustdoc, four launcher tests in the established WSL LF-normalized
packaging copy, Jekyll in 4.362 seconds and a 2m 24s optimized build/version smoke.
PR #3 is stacked on #2 and #1; protected main checks and GitHub review remain gates.

No manual authenticated playback, Linux/macOS application runtime or new CPU/RAM
benchmark is claimed. Unix/platform behavior remains a cross-platform CI concern.
