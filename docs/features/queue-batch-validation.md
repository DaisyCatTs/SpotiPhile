# Queue selection and Play Later

Date: 2026-10-10. Branch: `feature/queue-controls`. UI redesign remains paused.
This batch extends existing native views and does not change authentication,
stored schemas, dependencies or the audio engine. No push or merge is included.

## Behavior

Play Later appends selected playable tracks and episodes to the manual queue,
before the playlist/context continues. It preserves selection order and duplicate
copies, including intentional repeated invocations. It requires active local
playback and a matching displayed manual prefix. Remote Add to Queue retains its
inherited append behavior.

Upcoming queue rows have independent selection: click, Ctrl/Cmd toggle, Shift
range and Ctrl/Cmd+A. Copy retains displayed order and duplicate links. Cut and
Delete remove exact selected manual occurrences. Paste appends supported Spotify
track/episode links to the manual queue. Ctrl/Cmd+Up/Down moves the selected block;
native menus move it to either edge. Dragging selected manual rows moves their
occurrences as a block, retaining their relative order.

Editing requires every selected row to belong to the manual queue. A selection
including context rows can be copied but cannot be removed or reordered. Stale
queue snapshots, mismatched displayed prefixes, inactive playback and remote
targets cannot apply these edits. Text fields and dialogs retain their shortcuts;
a focused queue inspector owns its shortcuts independently from the main list.
Current playback and context continuation remain untouched by manual edits.

Occurrence mappings live in `src/queue_edit.rs`. `App::apply` validates actions,
updates pending occurrence indexes and remaps selection, then uses inherited
local clear/re-add resynchronization and stale-answer guards.
Selection tracking compares URI order before copying it, so an unchanged queue
does not allocate a new copy of every URI on each draw. No new performance
benchmark is claimed.

## Verification

Twenty-six new regression tests passed: ten application tests, ten native demo
tests and six pure mapping tests. Native tests exercise actual pointer, modifier,
keyboard, drag and menu events. App tests cover duplicate metadata, pending
indexes, context and paused playback, stale answers, capability guards and
clipboard ordering. Mapping tests exhaustively check small-list selections and
insertion slots.

Formatting, both Clippy configurations with warnings denied, default and
all-features test suites, doc tests and documentation build passed. Default:
995 library tests passed. All features: 1,018 passed. Each retains one ignored
native credential-store test. Binary/integration tests, including localization
completeness and placeholders, passed. All 14 catalogs contain the new labels;
existing translations were retained, with zero fuzzy entries, and catalog sync
check passed. Four launcher tests passed in WSL against the existing normalized
packaging copy. Jekyll built successfully in WSL.

The full suites also revalidated inherited Add to Queue, single-row drag,
clear-manual and context-preservation behavior. Relevant tests include
`a_song_picked_twice_is_queued_twice`,
`clear_queue_takes_the_hand_queued_rows_and_keeps_the_context`,
`clearing_a_just_queued_song_leaves_the_contexts_copy_of_it`,
`dragging_a_queued_row_within_the_queue_reorders_it`, and
`dropping_on_the_queue_without_local_playback_still_just_appends`.

The initial new demo tests failed because the pinned egui fork requires an
explicit `ModifiersChanged` event. The harness was corrected and the behavioral
assertions retained. Review also caught invalid-drag insertion fallback and
shortcut ownership conflicts; both were fixed and covered by native regressions.
An initial Clippy collapsible-if finding was corrected without suppressing it.
Independent final review found no remaining actionable issue.

The final Windows release build passed in 2m 22s. The resulting
`target/release/spotifast.exe --version` succeeded and reports `spotifast 0.12.0`;
the inherited executable identity/version remains deliberate. Full checks were
rerun after the final selection-tracking allocation change. No credentials or
live profile were opened for this smoke check. `git diff --check` passed.

## Changed boundaries

Implementation: `src/app.rs`, `src/model.rs`, new `src/queue_edit.rs` and its
module registration, existing queue/widget/collection UI, and player-bar drag
payload construction. Deterministic UI coverage is in `src/demo.rs`.
Documentation: this report, the queue contract, README, divergence ledger and
master checklist. Gettext catalogs carry three new labels in all 14 locales.
Earlier appearance and Play Next/edge changes remain preserved in the working
tree and are not attributed to this batch.

No live authenticated playback measurement or Linux/macOS native build is claimed
for this batch. Undo/redo, Pin Next, Session Capsules, subsection shuffle and
queue search remain unfinished; the overall Queue 2.0 checkbox remains open.
