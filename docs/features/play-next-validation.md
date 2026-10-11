# Play Next validation

Date: 2026-10-10. Branch: `feature/play-next`, based on `24e9f59` with existing
uncommitted Phase 1 work preserved. The user paused further UI redesign and
authorized queue functionality first. This is a local Play Next feature, not
completion of Queue 2.0 or Phase 2.

## Behavior and ownership

The existing single-row and selected-row context menus emit `Action::PlayNext`.
`App::apply` rechecks the target and playable items, then inserts the selection
at the front of the manual queue using the existing local reorder mechanism.
The current item, playback state and context rows remain unchanged. Selection
order and duplicate occurrences are preserved. Unavailable tracks and local-file
rows are skipped. Remote/inactive playback disables the menu action; an action
already emitted also becomes a no-op if local insertion is no longer available.

The existing target policy gives a ready, active local player precedence over a
selected remote device. This feature follows that policy rather than changing
device routing. A stopped local player is inactive; a paused player is active.

Play Next uses its own 1.5-second debounce for identical URI selections while the
manual queue remains unchanged, separate from Add to Queue. A deliberate copy
added immediately after an append survives.
Clear queue and sign-out reset this guard. Local insertion shifts pending indexes
and invalidates outstanding queue answers through the existing resync path.
Drag insertion retains its previous append fallback and per-item debounce.

No additional dependency, Spotify endpoint, permission, persistence format or
authentication behavior was introduced. MilkDrop and Winamp remain enabled.

## Verification

- Eleven tests matching `play_next` passed with all features. Nine were added for
  this feature; two are existing queue regression tests.
- New application tests cover order, duplicate occurrences, pending indexes,
  untouched paused/current playback, backend commands, invalid items, debounce,
  rejected remote/inactive actions and stale pre-insertion answers.
- New demo tests render the actual native menu body and send pointer clicks.
  They verify single/batch action payloads and disabled actions. They do not
  separately validate tooltip appearance or a full operating-system popup.
- Formatting and Clippy with warnings denied passed for default/all features.
- Full default suite: 961 library tests passed, one ignored; binary/integration
  checks also passed. All-features suite: 984 library tests passed, one ignored;
  binary/integration checks also passed. The ignored test requires a native
  credential-store round trip; credential storage was unchanged.
- Documentation tests and documentation build with warnings denied passed.
- GNU gettext extraction/update and read-only checks passed using the pinned
  dependency's LF-normalized script in WSL. An official Debian GNU gettext 1.0
  package supplied the tools from ignored cache, without system installation.
  All 14 catalogs passed format validation. Existing translations were retained,
  no fuzzy entries were present, and each catalog gained 17 source messages
  (including pending Phase 1 appearance labels). New translations were prepared
  by the assistant. Existing login/footer wording was carried forward with the
  product name replaced; the superseded branded entries were removed. The
  localization test validates complete coverage and named placeholders. Fluent
  speaker review of the new translations remains desirable.
- Four launcher tests passed in WSL using the existing LF-normalized copy of
  unchanged packaging files. The documentation site built successfully.
- The Windows release build passed with default features, including MilkDrop.

Windows is the verified Rust platform. These results are not Linux/macOS native
build coverage, a live Spotify playback test, or remote CI results. No real
Spotify profile was opened or modified for these tests.

Local logs are under `.cache/features/play-next/`; the focused test log is
`.cache/features/play-next/focused-tests.txt`. These ignored logs are
machine-local evidence, not distributed artifacts.

The initial inactive/remote fixture failure was corrected to model actual target
policy. Independent review also found an overly broad debounce after queue
consumption/removal; the final queue-snapshot guard and regression test resolve
it. The first local gettext build aborted during output, so extraction was
repeated successfully with the official packaged tool. Failed attempts are
described here; tooling build diagnostics also remain in local cache.
The first refreshed catalogs also failed the localization test: they introduced
empty translations and superseded branded entries. Filling the required entries
and carrying the old branded translations forward resolved that failure without
changing the test or its completeness requirements.
