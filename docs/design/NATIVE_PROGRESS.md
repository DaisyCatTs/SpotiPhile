# Spotiphile native foundations: current evidence

Updated 2026-10-09. Phase 1 remains in progress. Daisy rejected the replacement
prototype and native composition. Uncommitted source changes and captures are
development evidence, not an approved design or release.

## Development work

- Display name Spotiphile in the window, login and About. Package, profile,
  credential, updater and Spotify Connect identities remain inherited.
- Optional OLED, accent and row-density settings through existing Actions and
  persistence. Legacy Dark/Light/System values are preserved. Unknown or malformed
  new appearance fields fall back without discarding unrelated settings.
- Built-in neutral palettes and custom-theme precedence, with compatibility,
  contrast and row-stride regression coverage.
- Narrow navigation rail with the existing library in a popup. Saved expanded
  width is preserved. Popup list/grid dragging and autoscroll regressions now pass.
- Home section IDs avoid egui identity collisions. Collection headers and rows
  consume proposed density settings. Queue/Lyrics selection uses existing panel
  actions. The composition itself remains rejected.

## Recorded checks and limits

Ignored logs are under `.cache/phase1/`. The release and demo builds succeeded in
`native-build-release.txt` and `native-build-demo.txt`. Jekyll built successfully
in `native-jekyll.txt`. Linux launcher tests passed against an LF-normalized
scratch copy in `native-launchers-lf.txt`; this is not Linux native app coverage.

Final checks on 2026-10-09 passed: formatting; default and all-feature Clippy
with warnings denied; default tests (952 library tests passed, one ignored);
all-feature tests (975 library tests passed, one ignored); binary and integration
tests; doctests; documentation with warnings denied; and the Windows release
build (3m 16s). Logs are `final-*.txt` under `.cache/phase1/`.

Earlier grid-drop/autoscroll failures are retained in `native-rail-tests.txt`.
Their fixes preserve the egui layer/occlusion guard. All three popup regressions
passed in the final suite. The header fixture now locates the correct content
row while retaining its same-row and non-overlap assertions. Independent review
found no concrete regression in these focused changes. The credential-dependent
keyring test remains ignored. No fresh macOS or Linux native Rust build is claimed.

Actual Rust demo captures are in [native-captures](native-captures/): dark Home
1280×800, light narrow Playlist 760×800 and OLED Playlist 1440×900. They used
isolated fictional demo profiles. They do not establish visual approval or a
new performance baseline.

## Remaining gates

The appearance labels and renamed login copy are not yet synchronized into the
translation template and PO catalogs. The existing localization test compares
catalogs against that template, so passing it does not establish fresh source
coverage. Accent/density choices now use literal gettext calls for extraction.
The direct WSL extraction check hit checkout CRLF in the shell script; WSL also
lacks `xgettext`. Catalog extraction and reviewed translations remain required.

Popup regression closeout and final Windows checks are complete.
Establish a full-screen replacement composition with matching dark/light and
narrow evidence before further broad native styling. Complete Figma tokens and
reusable components when an available API supports those operations. Local
TalkToFigma supports editable primitive studies, distinct from bound variables
or component definitions.

No Phase 1 changes have been committed, pushed or merged. No Phase 2 work is
included. MilkDrop, Winamp and native Rust product boundaries remain.
