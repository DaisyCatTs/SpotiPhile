# Manual queue edge controls

Date: 2026-10-10. Work branch: `feature/queue-controls`. The user requested more
features with UI redesign paused. Earlier Phase 1 and Play Next work remains
preserved; no commit, push or merge is part of this change.

## Implemented behavior

Existing native queue-row context menus offer **Move to top** and **Move to
bottom** for manually queued rows. Moving one occurrence preserves the relative
order of every other row, including duplicate songs. The current item,
paused/playing state and context continuation remain unchanged. An occurrence
already at the requested edge cannot issue that move.

The controls require active local playback and an exact match between the
displayed manual prefix and `manual_queue`. Remote/inactive targets and partial
or stale displayed queues cannot issue a move; context rows do not offer the
controls. The action includes the shown manual queue snapshot and rechecks it
and local capability when applied, preventing an old index from targeting a
changed queue.

`Action::MoveQueueToEdge` delegates to the existing `MoveInQueue` implementation.
Its pending-occurrence index shifts, optimistic queue display, persistence dirty
flag and stale-answer protection are retained. No new Spotify endpoint,
permission, setting, stored format, dependency or player command was introduced.
Remote Add to Queue still appends. MilkDrop and Winamp are preserved.

## Verification

Eight focused application/native-menu tests cover:

- Both endpoints, duplicate occurrences, pending indexes and untouched context.
- Current track and paused state; exact local clear/re-add command order.
- Same-edge no-ops; remote/inactive targets and context-row exclusion.
- Changed queue snapshots, mismatched rows, partial prefixes and stale order.
- Stale queue answers after a move.
- Actual menu pointer clicks, emitted snapshot/index/edge payloads and applying
  those actions through `App::apply` to verify displayed and manual order.

Formatting, both Clippy configurations with warnings denied, default and
all-features suites, documentation tests and documentation build passed. Default
suite: 969 library tests passed; all features: 992 passed. Each leaves one native
credential-store round-trip test ignored because credentials are unchanged.
Binary/integration checks, including catalog completeness and placeholders,
also passed. Four launcher tests passed in WSL using the existing LF-normalized
copy of unchanged packaging files. The documentation site built successfully.
The Windows release build passed with default features, including MilkDrop.
Local evidence is under
`.cache/features/queue-controls/`; ignored logs are machine-local artifacts.

Three new labels were translated in all 14 catalogs. Existing translations are
preserved and no fuzzy entries were introduced. These are assistant-prepared
translations; fluent-speaker review remains desirable. GNU gettext extraction
and format checks use the official packaged tools already prepared in ignored
cache for Play Next.

Coverage is Windows Rust tests/build and headless native menu interaction. This
does not claim a live authenticated playback test, operating-system popup
capture, Linux/macOS native compilation or remote CI results. No real Spotify
profile was opened or changed during verification.

Optional cleanup of unused GNU gettext 0.26 bootstrap sources was rejected by
local tool policy (the only stated reason was "blocked by policy"). Those files
remain in ignored cache; validation uses the separate official packaged tools.
