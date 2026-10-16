# Recent search typing-run deduplication

Pausing while typing no longer leaves every successful prefix in Recent
searches. Extending or shortening a query replaces only the history entry
added by that same run of typing. Existing entries remain independent.
Clearing the field, replacing selected text, pressing Enter, choosing a
recent search, or opening a search link ends the run.

## Source and adaptation

- Upstream: crmne/spotifast, PR #747 (fixes #741).
- Original commit: `4ebfcf06931519de76733fa1048523445c3fcbc2`.
- Original author: Samet Karaş
  `<115280393+SametKaras@users.noreply.github.com>`.
- Ported the cached commit's in-memory run tracking, history ownership,
  selection detection, and three regression tests to SpotiPhile.
- Adaptation: Enter emits `SubmitSearch`; existing recent-search chips and
  links retain `Search`. Query equality cannot identify an Enter submission.
  A recent chip identical to an in-flight typed query must clear that query's
  run ownership even when the existing request is reused, preserving the
  earlier prefix as a separate entry.
- Added coverage for that identical-chip case and both arrival orders of
  catalogue/playlist answers, including stale replies.

The change adds no dependencies, requests, identity changes, persistence
schema changes, or layout, window chrome, and theme changes. Existing queue
controls and login-port handling are outside this patch.

## Validation status

The implementation agent inspected AGENTS.md, CONTRIBUTING.md, the connection
reference, nearby search tests, and all search action producers. Only the
three owned Rust files were formatted with installed Rust 1.98.0 rustfmt.
`git diff --check` passed.

Parent Windows integration passed 1,038 default and 1,062 all-feature library
tests (two existing ignored tests each), all target suites, formatting, both
warnings-denied Clippy configurations, doctests and warnings-denied Rustdoc.
The optimized build and version smoke passed. Two additional native egui tests
passed: Ctrl+A replacement ends the run through a deferred action, continued
typing keeps the new run, Enter submits, and a Recent chip emits a separate
search. The complete all-feature suite then passed 1,064 library tests (two
existing ignored), followed by all-feature Clippy and an unchanged release smoke.
Jekyll and final diff checks passed. Independent source review found no blockers.
No macOS/Linux runtime coverage or authenticated search is claimed.

New regression tests in `src/app.rs`:

- `pauses_while_typing_record_only_the_final_query`
- `a_new_query_or_a_finished_search_keeps_its_own_recent_entry`
- `typing_never_replaces_a_recent_search_it_did_not_add`
- `an_identical_recent_search_does_not_submit_the_typing_run`
- `typing_history_handles_split_answers_and_rejects_stale_replies`

Native input regressions in `src/demo.rs` complement the state tests:
`replacing_selected_global_search_ends_its_run_after_drawing` and
`global_search_enter_submits_but_recent_search_click_searches`. These exercise
the actual TextEdit and chip widgets. Authenticated catalogue/network latency
and non-Windows runtime behavior remain unverified locally.
