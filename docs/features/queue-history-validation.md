# Local queue undo/redo

Date: 2026-10-11. Branch: `feature/queue-controls`.

Undo/Redo buttons and Ctrl/Cmd+Z, Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y reverse manual
queue removals, clear, moves, inserts and immediate additions. Current playback
and context continuation are preserved. Selection and duplicate row metadata
are restored; pending local occurrences receive fresh indexes and timestamps.
Remote write IDs are never restored. Text editors, dialogs and popups retain
their shortcuts. The inspector keeps its narrow tab header unchanged.

History exists only in memory. Next, Previous, track sequence changes (including
another occurrence of the same URI), loading/stopped playback, disconnect,
transfer, sign-out, playback requests and shuffle invalidate it. Each restore
also checks the current song, sequence, exact manual prefix and context suffix.
External changes therefore disable stale history. Matching metadata refreshes
do not invalidate it. Delayed queue answers cannot reverse a restore.

The journal retains at most 20 edits and 8,192 rows of snapshots/context
signatures. Queues exceeding 4,096 upcoming rows remain editable but do not record
undo snapshots. Background album additions are not undo entries; a later
asynchronous change disables history whose expected queue no longer matches.
Undo of Clear does not restart an album request that Clear intentionally cancelled.

Six focused app/native regression tests passed for round trips, ordered copies,
selection/pending indexes, multi-step history, redo invalidation, no-ops, bounds,
same-song advancement, remote/disconnect/context guards, delayed answers and
actual button/shortcut events. The first broad run caught header wrapping and
an obscured translated Recent control; history buttons were moved out of the
inspector tab row, and the original assertions passed unchanged.

Both Clippy configurations and complete suites passed before final safety/limit
refinements; final combined checks and production build are recorded below when
finished. No live authenticated playback or Linux/macOS runtime test is claimed.
