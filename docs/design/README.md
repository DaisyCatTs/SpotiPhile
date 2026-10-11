## Current refinement, 2026-10-10

[Interactive proposal](http://127.0.0.1:8766/design/professional-shell.html) and
[before/after comparison](http://127.0.0.1:8766/design/refinement-comparison.html)
are ready for review. The previous 25:2 Figma composition remains rejected.
This revision is not approved and has not been carried into native Rust.

The new pass uses neutral graphite surfaces, a bounded unboxed artwork shelf,
collection thumbnails, calmer type weights, integrated Home tracks, a compact
playlist header and a single Queue/Lyrics inspector. Decorative gradients and
an intermediate duplicated now-playing block were removed. MilkDrop and Winamp
remain visible. Dark, Light, OLED, accent and density choices are preserved.

Prototype filters now narrow the displayed fictional tracks and provide an empty
state. Named selection controls expose pressed state; Space and Ctrl/Command
selection preserve modifiers. Narrow filters remain available. These local
interactions are design demonstrations, not real Spotify playback/search.

[Validation](refinement-validation.json) records 27 page/theme/viewport cases,
filter/selection/inspector checks and no page JavaScript errors. An independent
review found three interaction issues; all were corrected and checked.
[Captures](refinement-captures/README.md) include matching content and requested
sizes. Before images came from the shared preview and may be scaled; after
images are exact-size headless captures. Do not attribute sharpness differences
to the design. The shared preview host disconnected; verification used installed
Playwright Chromium only after its explicit unavailable error.

Figma frame 25:2 has a rejection annotation, recorded in
`figma-rejection-ledger.json`. Official use_figma still reports its tool limit.
The latest visual proposal is HTML evidence; no new full Figma screen, bound
variables or reusable components are claimed.

> **Latest visual review:** Daisy rejected the replacement and native composition.
> Spotiphile and themes are retained. See [the professional design reset](PROFESSIONAL_DESIGN_REVIEW.md).
> The older `research-shell.html` is rejected visual evidence, regardless of its
> passing geometry checks. Native changes remain uncommitted development work.

# Spotiphile Phase 1 design workspace

**Design reset, 2026-10-08:** Daisy rejected the first visual draft. Its cream/gold
palette, first-generation Figma frames and HTML review are rejected history,
regardless of their structural test results. See [design reset](DESIGN_RESET.md).
Replacement research is recorded and native implementation has started after
Daisy authorized continuation. Do not implement the old tokens.

**Product name:** Spotiphile, selected by Daisy. See [native progress](NATIVE_PROGRESS.md)
for actual runtime changes, captures and remaining work.

Current work: [comparative UI research](UI_RESEARCH.md),
[native implementation sequence](NATIVE_IMPLEMENTATION_PLAN.md), and an editable
[research board in the canonical Figma file](https://www.figma.com/design/QY3qv8UVxL4PhHPEs0xHJo/?node-id=8-438).
The board is on the existing active page. It does not complete the required
Figma page inventory or token/component system.

## Rejected replacement review history

[Replacement music workspace](research-shell.html) has been inspected and checked
as a review artifact. It covers Home, Library and Playlist in one native implementation
proposal. Its HTML is a local review artifact; the production app remains Rust
and egui. The new artwork is [original fictional review artwork](research-assets/README.md).
The [closeout checks](research-closeout-validation.json) record 864 combinations
of page, base theme, accent, density, inspector state and four viewport sizes.
They check horizontal overflow, player visibility and review-header control fit;
they do not establish usability for every combination. Targeted inspector
close/reopen, focus return, tab arrow keys and Escape checks passed. Local Inter
and artwork loaded with no broken images in the five current captures.
[Sampled contrast checks](research-validation.json) cover twelve base/accent
combinations. Neither record is a full accessibility certification.

The current captures have a `current-` prefix. Earlier `research-` captures are
intermediate evidence and do not include the final header/keyboard fixes. The
[editable theme study](https://www.figma.com/design/QY3qv8UVxL4PhHPEs0xHJo/?node-id=11-459)
is in the canonical Figma file. The earlier channel `k03dbyvo` was joined and the same
document roots were verified on resuming. These studies are not variable-bound
components. Daisy authorized native implementation after the replacement review.
The HTML remains review evidence; native progress is recorded separately.

Both that replacement and its native composition were subsequently rejected.
Channel `aszfrbr6` was verified on 2026-10-09. See
[Figma capability status](figma-foundations-status.md) for the current tool
boundary and [the second reset](PROFESSIONAL_DESIGN_REVIEW.md) for new review gates.

Initial review failures included a clipped header, a blank pause glyph, low
muted-label contrast, inert keyboard stops and incomplete inspector-tab semantics.
Those were corrected before the closeout checks. Native rendering, AccessKit
behavior, backend integration and performance remain implementation-stage checks.
The prototype-only closeout preceded the native changes. See native progress
for fresh Cargo checks and native captures. Phase 1 remains in progress.

Status: in progress. Phase 1 was explicitly authorized by Daisy on 2026-10-07.
Canonical specification: [master plan](../../DaisySpoti_MASTER_PLAN.md), sections
34-38, Phase 1 and Task C. Phase 0 is closed, with cold startup explicitly waived.

Canonical Figma file supplied by Daisy:
[DaisySpoti — Product & UX System](https://www.figma.com/design/QY3qv8UVxL4PhHPEs0xHJo/DaisySpoti-%E2%80%94-Product---UX-System).
An earlier blank draft was created before that link arrived. It is not canonical
and has not been deleted. The official Figma MCP connector hit its Starter-plan
call limit. Daisy then supplied TalkToFigma and a local channel; a local
stdio MCP client joined the user-supplied channel and inspected the empty page, selection,
styles and components. No existing user design was replaced.

Three editable shell drafts were then created on the active page: dark/light
1280x800 and OLED 760x800. [Frame IDs](figma-draft-frames.json), the exact
[operation ledger](figma-draft-ledger.json) and [structural audit](figma-draft-validation.json)
are retained. The drafts contain 159/159/117 editable frame/text nodes,
all text reports Inter, and no child bounds overflow their immediate parent.
PNG exports were visually inspected. One narrow brand label was repaired.
These are layout drafts, not completed components or variable-bound tokens.

## Rejected first-generation archive

[Rejected shell review](phase1-shell.html) and [rejected before/proposed comparison](review.html)
are standalone local artifacts. [Component contracts](COMPONENT_CONTRACTS.md)
describe the intended native boundaries and states.
These links preserve the old review record and are not current design input.
It can be opened directly or served from this directory. It is not part of
the Rust application and introduces no runtime browser dependency.

## Scope and approval

Prepare foundations, proposed tokens, reusable component specifications and a
representative shell review. Cover navigation, player bar, optional inspector,
dark/light/OLED, density, loading/empty/error states and keyboard focus.
The review prototype uses fictional data. It is not a playback client.

The master Task C says: “Wait for approved Figma/tokens before broad visual
implementation.” Daisy authorized continuation after the replacement review.
The old first-generation colors remain rejected. No Phase 2
queue/shuffle, local library, capability model or audio expansion is included.

## Rejected first-draft foundations

[tokens.proposed.json](tokens.proposed.json) defines semantic theme colors,
spacing, radii, motion, density and initial panel geometry. Figma uses Inter,
which the native client already ships. HTML requests Inter and falls back to
the platform font when unavailable; it does not embed a new font asset.
Geist remains a candidate requiring
native metrics, fallback and rendering validation before adoption.
[contrast-proposed.json](contrast-proposed.json) records 18 selected flat-color
text and focus pairs. Passing these pairs is not a complete accessibility audit.

Dark uses neutral graphite with a restrained warm accent; Light uses neutral
paper surfaces; OLED uses black surfaces. Artwork accent remains optional.
Compact/default/comfortable row heights are 38/46/54 logical pixels. At narrow
widths the workspace takes priority and the inspector becomes an overlay or
closes. Sidebar collapses to a rail. Draft review must cover 760, 1280 and
1440-pixel widths and all three themes before approval.

Figma, proposed tokens and interactive review use 200/272 sidebar/inspector
geometry. The panels remain user-resizable in the native implementation proposal.
The initially wider HTML draft was aligned before the final review captures.
The review uses existing page/queue/lyrics semantics; future Releases and other
roadmap labels in a mockup are design context, not an implementation claim.

## Rejected first-draft verification, 2026-10-08

[Prototype validation](prototype-validation.json) records 27 combinations of
760/1280/1440-pixel width, dark/light/OLED and all three densities. Every case
had no horizontal overflow, the expected row height and at least three rows
above the persistent player. Inspector close/reopen, keyboard tab selection,
Escape/focus return, manual loading/empty/error selection and reduced motion
passed. No runtime errors or external requests were observed.

T3's shared preview initially worked, then explicitly reported no automation
host and instructed not to retry. Headless Chromium was used for the remaining
checks. The first headless check found narrow controls hidden, which was fixed;
the final checks pass. Earlier screenshot attempts caught intermediate CSS
transition colors; final captures disable animations to record settled states.
Figma and HTML exports were visually inspected. This does not certify full
accessibility, native font rendering, backend integration or production readiness.

At the time of this rejected first-draft review, the native executable was
unchanged. Current native work is recorded above. Reusable Figma components,
token variable bindings and the exact page inventory remain outstanding;
primitive draft frames do not satisfy them.

## Native implementation seams

- `src/ui/mod.rs::show` composes the shell; `central` routes existing pages.
- `src/theme.rs` owns palettes, typography, icons and primitive styling.
- `src/ui/sidebar.rs`, `topbar.rs` and `player_bar.rs` emit existing Actions.
- `src/ui/queue.rs` and `lyrics.rs` already provide mutually exclusive, closable,
  resizable right panels. The draft inspector retains one active tab at a time.
- `App::frame_ui` draws, then drains Actions through `App::apply`. Backend work
  remains on the existing runtime. Preserve optimistic queue/playback behavior.
- Existing theme enum and compact booleans need backward-compatible defaults
  if expanded. Do not rename profile/credential/package identities in Phase 1.
- Winamp, MilkDrop, window chrome, panel yielding and keyboard shortcuts remain
  protected regression surfaces. Review native captures after implementation.

## Validation gates

Figma must retain editable hierarchy, reusable components and token bindings;
primitive draft frames alone do not satisfy those requirements. TalkToFigma's
advertised tool catalog provides auto-layout and primitive creation but no
page/variable/main-component creation. Record that limitation explicitly rather
than checking off a complete Figma design system.

After design approval: introduce the smallest native token/component change,
preserve Action semantics and settings read compatibility, run appropriate
layout/settings/theme tests plus full contribution checks, and capture matching
before/after dark/light views at representative widths. Record approval of the
concrete design separately from compilation and behavior checks.
