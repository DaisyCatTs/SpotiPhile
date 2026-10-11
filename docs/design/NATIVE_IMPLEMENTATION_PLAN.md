# Native implementation sequence after design review

Status: Phase 1 in progress. Daisy authorized implementation after the
replacement review and Spotiphile name decision, then rejected its composition.
Further broad native styling awaits a stronger full-screen review. Read with the master plan
Task C and the replacement research review. Native progress and actual evidence
are recorded in [NATIVE_PROGRESS.md](NATIVE_PROGRESS.md).

## 1. Theme and density contracts

Extend `src/theme.rs` using the existing `Palette`, Inter weights, Lucide icons
and `fastframe-theme` catalog. Introduce semantic geometry and density values
only where the approved components consume them. Keep base theme independent
from accent. Theme files must continue to load; do not replace the custom-theme
schema wholesale. In `src/settings.rs`, preserve the System/Light/Dark serialized
values and defaults. Prefer separate optional Daisy variant/accent/density fields
with serde defaults; OLED can retain `theme: "dark"` as the old-reader fallback.
Naively adding a new serialized `ThemeChoice` enum value would make an older
reader reject the whole settings object. The current loader falls back to all
defaults on parse failure. Tolerant deserialization in our new reader alone
does not fix an older binary's parser. Verify round trips through representative
old and new readers, unknown Daisy values, malformed values and unrelated
settings preservation. Do not infer actual platform Mica support from a theme name.

Wire controls through existing settings Actions and save paths. Never let a view
write files or perform network requests during drawing. Test malformed custom
colors, theme switching and contrast-critical foreground/background pairs.
Compare native font/fallback rendering before choosing any new font dependency.

## 2. Primitives

Update the existing button, icon button, navigation, tabs, table-row, search and
focus styling. Reuse `theme::apply` so menus/dialogs agree with custom views.
Keep focus distinct from current track and selection. State must have text,
icons or geometry as well as color. Preserve bidi handling and translated text.
Validate long names and scripts outside Inter's coverage in native captures.

## 3. Shell and navigation

Change `src/ui/mod.rs::show` and existing sidebar/topbar modules. Preserve the
`App::frame_ui` draw-then-apply sequence. The shell owns layout; views emit existing
Actions. Route existing pages without changing their data contracts. Keep pins,
folders, history, search and settings reachable; do not remove useful upstream
functionality because the new default surface is simpler.

Use current panel-yielding and persistence helpers. New rail behavior must not
write a temporary narrow width over a user's remembered expanded width. Do not
break Windows custom chrome/resize or macOS titlebar insets. Do not introduce
per-frame disk work or continuous repaints for static shell decoration.

## 4. Player and inspector

Restyle `src/ui/player_bar.rs` around existing playback Actions, seek/volume
widgets, media state and cover loading. Preserve spectrum/oscilloscope and
art-tint preferences, post-EQ/pre-volume signal semantics, remote capabilities
and truthful output state. Default appearance must not silently change processing.

Compose Queue/Lyrics presentation around existing mutually exclusive panels.
Keep close/reopen and width persistence behavior. Preserve their center-page
routes. Additional Info/Signal Path/Device tabs are implemented only if their
actual current data and Actions support them; otherwise omit them from the
Phase 1 runtime. Do not implement Queue 2.0 under an inspector refactor.

## 5. Existing page presentation

Apply approved Home, Library and Playlist composition to existing data models.
Keep collection sorting, pagination, selection, context menus, drag/drop,
optimistic actions and loading/error/cache behavior. Replace mock album graphics
with the existing Spotify cover pipeline; no external review asset service or
browser runtime belongs in the application.

## 6. Verification and review

Run formatting, required lint/static checks, full appropriate test suite and
production/demo builds from CONTRIBUTING. Add regression coverage for changed
settings compatibility, layout transitions and behavior, rather than tests
that simply repeat constants. Review native captures at narrow/standard/1440p
sizes, all themes, inspector open/closed and failure states. Exercise keyboard,
Winamp, MilkDrop and player-bar visualization access. Record platform coverage
honestly and compare runtime measurements if changed repaint/layout behavior
warrants profiling.

Inspect AccessKit roles, accessible names and states after replacing navigation,
transport buttons, table selection/current rows, inspector tabs and theme/density
controls. Keyboard navigation and visually obvious icons do not establish
screen-reader behavior. Check disabled/toggled state, focus return and the
meaning conveyed by selected/current indicators as well as rendering.

Review the final diff for unrelated auth, backend, playback-engine or packaging
changes. No stable-main merge, package/profile identity migration, dependency churn or
later-phase feature expansion is part of this sequence. Mark Phase 1 complete
only after the new native shell drives the existing backend and required checks
and visual review have actually passed.
