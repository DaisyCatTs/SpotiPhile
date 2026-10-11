# DaisySpoti divergence ledger

Baseline: `995c768dcba4da7f93302bf2107af6b7ec3df02b`.

## Intake divergence

None. The entire tracked tree and history matched fetched crmne/spotifast main
at intake. No Daisy UI, shuffle, local music, audio graph, branding or protocol
modifications existed. MilkDrop, Winamp, patched librespot, fastframe, auth,
caches and optimistic queue logic are inherited upstream functionality.

## Phase 0 divergence

| Surface | Ownership / change | Validation |
| --- | --- | --- |
| Master plan, handoff, Figma brief | Daisy specification, copied verbatim from attachments except validated master checklist updates | Compared before checklist changes; later phases untouched |
| `docs/upstream/*` | Daisy baseline evidence, ledgers and audit | Git/source references and executed commands in validation report |
| `docs/adr/*` | Daisy decisions already mandated by the brief or inherited implementation | Checked against current source and explicit requirements |
| `.github/workflows/ci.yml` | Push CI also runs on dev | One-line trigger change; no runtime or test changes |
| `AGENTS.md`, `CONTRIBUTING.md` | Explicit Daisy policy override, inherited implementation rules preserved | Removes future direct-to-main policy ambiguity |
| Test/developer executable names | `launch_contract` and `release-inspect`, same contents | Full Cargo checks recorded in VALIDATION.md; avoids Windows error 740 |
| `.github/workflows/phase0.yml` | Read-only Daisy formatting/manifest/packaging checks on dev/manual dispatch | Push and manual runs pass; full CI registration now works |
| Git configuration/refs | Added upstream; published baseline tag and local namespaced release tag; local/remote dev | Exact ref checks in baseline |
| GitHub protection | Stable-main review gate and linear/no-force/no-delete safeguards | Read-back snapshots in evidence |

No Cargo manifest, lockfile, runtime source, audio, queue or visual behavior was
changed in Phase 0. Documentation and CI are the first Daisy-owned work.

The policy/CI change is committed as
`a073de4f3e7367c021294c65e0b6293167608613` on dev. Main remains at the baseline.

## Development feature divergence (2026-10-10)

The user paused further UI redesign and authorized queue features first.
`feature/play-next` preserves the pending Phase 1 changes and adds a Daisy-owned
local Play Next action. `src/model.rs`, the action fold and insertion helper in
`src/app.rs`, and the existing menus in `src/ui/widgets.rs` are its integration
boundaries. The helper retains upstream drag-insert fallback/debounce behavior;
Play Next independently checks capabilities and never becomes remote append.
Occurrence indexes, optimistic queue state and stale-answer guards remain
supplier-sensitive invariants. Review those paths when porting future upstream
queue fixes. This feature changes no Spotify request, playback engine command,
credential namespace or stored schema. It is not an upstream port.

Application/menu regression tests, the queue contract, README, master checklist
and gettext catalogs accompany the feature. Catalogs also synchronize pending
Phase 1 labels; existing translations are preserved. See
[Play Next validation](../features/play-next-validation.md) for executed checks
and platform limits. The broader Phase 1 visual proposal remains unapproved.

The subsequent `feature/queue-controls` work adds manual **Move to top/bottom**
menus and `Action::MoveQueueToEdge`, delegating to upstream's existing positional
move action. Its full displayed-prefix and action snapshot guards are Daisy-owned;
pending occurrence shifts and queue resync remain inherited integration boundaries.
See [queue edge validation](../features/queue-edge-validation.md). It changes no
Spotify request, stored schema or player engine command.

## Future ownership boundaries (recommendation, not implemented)

The original persistence safety fix `f5a946c` is now deliberately ported as
`4c62229`; see the accepted ledger. Its source retains upstream author credit.
The renamed origin is `DaisyCatTs/SpotiPhile`. The immutable intake baseline
remains unchanged; the later fetched head and remaining review inventory are in
`UPDATE_REVIEW_2026-10-11.md`.

Queue selection, occurrence mapping (`src/queue_edit.rs`), batch edit actions,
clipboard/keyboard ownership and local Play Later are also Daisy-owned. They
reuse inherited optimistic queue reconciliation and local engine clear/re-add.
When porting upstream queue or collection changes, preserve exact selected
occurrences, pending indexes, context continuation, snapshot guards and focus
ownership. These changes introduce no auth or persistence migration. See
[queue batch validation](../features/queue-batch-validation.md).

Daisy owns product semantics, design system, UI, capabilities and its future
library overlay. Spotify protocol compatibility, protected credential mechanisms,
native output/platform fixes, Winamp/MilkDrop compatibility and upstream bug fixes
remain supplier-sensitive. An ownership label does not authorize removing useful
upstream behavior.

Highest conflict exposure: app.rs action/event fold; backend.rs auth/request
routing; model/settings persistence types; player/sink/vis signal path; queue
recovery and occurrence identity; shared egui/winit/fastframe/librespot revisions;
branding/app IDs/credential namespaces/updater/package names.

Before each port, compare behavior, persistence, network policy, queue invariants,
platform cfgs and dependency revisions. Port logic separately from upstream
appearance. Update docs, tests and ledgers together. Keep SHA-pinned patch groups
coherent; a compiling partial dependency update can still break native integration.

Audio reliability ports on 2026-10-11 preserve the inherited sink architecture.
The queue-feature branch now carries upstream's decoder wakeups and device
recovery, independently of the focused protected promotion branches. No Daisy
queue code or visual scope was replaced. See ACCEPTED_UPSTREAM.md for source SHAs
and docs/features/upstream-audio-validation.md for combined verification.
