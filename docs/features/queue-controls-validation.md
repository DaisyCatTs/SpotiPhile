# Manual queue controls

This focused development candidate extracts completed queue features without
the paused interface redesign. It retains the existing playback context,
native rendering, and account/profile formats.

**Play Next** inserts playable selected occurrences at the head of the manual
queue. **Play Later** appends them to its tail, before the playlist or album
continues. Neither starts a new song or changes paused playback. Selected order
and repeated occurrences survive both operations. Play Next suppresses the same
selection repeated within 1.5 seconds while the manual queue is unchanged;
Play Later and clipboard paste allow intentional repeated additions.

Manual rows support top/bottom moves, selection, range and toggle selection,
block dragging, removal, copy/cut, and pasting Spotify track/episode links.
Edits recheck the active local player and expected manual queue at application
time. A stale selection cannot rewrite a different occurrence or a context row.
Another Connect device keeps its existing append-only Add to queue behavior.
Play Next/Later, positional editing, removal, paste, and Undo/Redo are local-only.

Search matches loaded titles, artist/show subtitles and track album names,
case-insensitively. It preserves original queue indexes and play order, does
not fetch unseen context pages, and limits range/Select All to visible rows.
Hidden rows leave the selection. Positional drops, keyboard moves and Clear
queue are unavailable while filtering; an explicit top/bottom menu action still
uses the whole manual queue. Cached results are reused until the query or
metadata changes, and clearing search releases its retained metadata copy.

Undo/Redo restores manual occurrences and selection only while the current
track sequence, active local player and context still match. Next/Previous,
device transfer, shuffle and incompatible playback/context changes invalidate
history. A new edit discards Redo. History is session-only and retains at most
20 edits and 8,192 combined before/after/context entries. Snapshots are omitted
when the loaded upcoming queue exceeds 4,096 rows; ordinary edits still work.
Restoration creates fresh pending additions in restored order rather than
reviving old write IDs or positions.

## Validation

- Windows x64 checks passed: 1,028 default and 1,052 all-feature library tests,
  with two existing ignored tests in each configuration, and all target suites.
- Formatting, both warnings-denied Clippy configurations, doctests and
  warnings-denied Rustdoc passed. The optimized build and `spotifast 0.12.0`
  version smoke passed.
- Application regressions cover duplicate occurrences, pending indexes,
  stale answers, inactive/remote targets, context boundaries, debounce,
  selection/filtering, clipboard input, and bounded history.
- Deterministic native demo tests cover menus, buttons, keyboard focus,
  filtered selection and block drag payloads, including text-editor and
  collection-shortcut isolation.
- The initial new edge-move Undo regression incorrectly expected old pending
  positions. Inspection confirmed restoration deliberately rebuilds them. The
  corrected test checks fresh indexes, matching occurrence URIs and absent old
  write IDs; the complete suites then passed.

## Limits

Authenticated interaction with Spotify has not been exercised for this
revision. Windows checks do not establish macOS/Linux runtime behavior or
protected remote CI success. Search covers loaded rows only; Undo does not
reverse playback, recover consumed songs, or persist across application restarts.
No new dependency, Spotify grant, credential handling or persistence schema is
introduced. See the [queue contract](../_reference/queue.md).
