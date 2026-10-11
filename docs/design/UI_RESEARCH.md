# Desktop music UI research, 2026-10-08

Status: research and replacement review in progress. The first draft is rejected.
This is a product design investigation, not a claim that one player is objectively
the best or that Daisy has inherited another player's audio capabilities.

## Question and method

How can a fast native Spotify client serve both everyday listening and serious
library work, while preserving Winamp, MilkDrop and truthful playback information?
Compare official desktop product documentation and published interface images.
All references were accessed on 2026-10-08. The Spotify redesign is dated
2023-06-20, Plexamp release notes 2026-09-01, and Columns UI release 2026-09-22.
Publication/update dates for other live help/product pages were not established;
they are undated references. Image dates are recorded separately where known
or explicitly unknown in the evidence manifest. Screenshots show a
particular documented state, not every current release or a usability study.
Use visual observations and documented behavior separately from our design
recommendations. Do not rank interfaces by marketing awards or subscription cost.

The initial draft failed our own brief: anonymous empty artwork, weak music
identity, explanatory prototype copy inside the product, and an unsolicited warm
accent. Passing layout checks could not correct those failures.

The repository's native Phase 0 [desktop capture](../upstream/evidence/desktop-dark.png)
and [narrow light capture](../upstream/evidence/narrow-light.png) were also inspected.
The desktop view already has useful artwork, transport and contextual tracks.
In the narrow capture, the expanded sidebar and queue leave very little central
space: the title becomes heavily truncated and top-bar controls overlap. This
is a concrete layout problem to resolve, not a reason to discard existing
features. Narrow inspector behavior must protect the workspace and keep essential
controls readable.

## Source comparison

| Reference | Evidence | Useful lesson for Daisy | Boundary / tradeoff |
| --- | --- | --- | --- |
| Roon | [Albums](https://help.roonlabs.com/portal/en/kb/articles/albums) documents library filter, Focus and sorting, with album and browsing images. The album image was inspected locally. | Give album identity a strong cover/title/artist relationship, keep play and queue intent legible, put tracks below contextual album information. | Its rich metadata and discovery services are not our Phase 1 scope. Do not imitate its large album header on every table page. |
| Roon | [Signal Path](https://help.roonlabs.com/portal/en/kb/articles/signal-path) explains an on-demand footer entry and the uncertainty after handing audio to hardware. | Audio information belongs one deliberate step away from transport. Describe observed facts and unknowns honestly. | Do not copy lossless indicators, quality-color classifications or hardware certification into Spotify playback. Signal Path implementation is later-phase work. |
| Apple Music for Windows | [Column browser](https://support.apple.com/en-ca/guide/music-windows/musd4821f613/windows), including the inspected official screenshot, uses category filtering above a song table. [MiniPlayer](https://support.apple.com/en-gb/guide/music-windows/mus71d7dcfce/windows) keeps artwork, playback and upcoming songs together. | Stable columns, predictable alignment and local filtering serve large libraries better than endless tile grids. Browsing and listening can have different density. | Do not reproduce every column or expand navigation into a second entire library. No new mini-player behavior in this scope. |
| Audirvana Studio | [Studio page](https://audirvana.com/audirvana-studio/) describes its 3.0 redesign, contextual controls and settings panel. Official [interface image](https://audirvana.com/sitev8/wp-content/uploads/2026/05/Interface.png) and [desktop product image](https://audirvana.com/sitev8/wp-content/uploads/2025/12/Studio-01-1024x768.jpg) were inspected. | Separate music browsing from advanced audio configuration; distinguish processing steps and selected tools with restrained surfaces. | Its processing, device and streaming capabilities are not ours. The desktop product image is angled promotional photography, not adequate for measuring pixel geometry. |
| Spotify desktop | [2023 desktop redesign](https://newsroom.spotify.com/2023-06-20/spotify-desktop-experience-redesign-your-library-now-playing-views-customize/) describes adjustable Library and Now Playing views and compact library options. [Sort/filter](https://support.spotify.com/sc-en/article/sort-and-filter/) documents collection navigation. | Keep familiar playback and queue intentions, allow optional side panels to yield space to the center, retain sorting and filtering. | Historical redesign documentation is not proof of the exact 2026 screen. A Spotify clone would not solve Daisy's identity or desktop density goals. |
| Plexamp | [Official product page](https://www.plex.tv/en-gb/plexamp/) describes themes, artwork treatment and visualizers. The web crawler returned 403; a normal HTTP fetch subsequently succeeded. Its [published mobile player image](https://www.plex.tv/wp-content/uploads/2022/09/plexamp-section-iphone-1-1024x1445.png) was inspected for album-aware color and track/art identity. | Artwork can supply character while themes and visualization remain user choices. | This is mobile promotional imagery, not evidence of desktop geometry. Product descriptions do not establish Daisy capabilities. Avoid automatic full-window coloration that harms table contrast. |
| MusicBee | [Official skin catalog](https://getmusicbee.com/addons/s/?type=1) includes multiple community styles. The official [main image](https://getmusicbee.com/img/hero_img/hero-img-top-min.png) and [skin illustration](https://getmusicbee.com/img/hero_img/hero-img-skin-min.png) were inspected: cover shelf, contextual track area and separate track information. | Theme ownership matters to music-player users; cover browsing and focused track work can coexist. | These promotional images are undated. A community skin catalog does not establish the current default UI or imply that arbitrary skins are safe or accessible. |
| foobar2000 | [Official Windows screenshots](https://www.foobar2000.org/screenshots-windows) identify dark/light album lists, visualizations, library search, layouts and preferences. The album list and example layout images were inspected: dense hierarchy, grouped tracks, artwork and a spectrum. [Columns UI](https://www.foobar2000.org/components/view/foo_ui_columns) separately documents modular panels and grouped playlists. | Dense task-specific views and customization deserve preservation. Keep readable type and a simpler default hierarchy than a fully populated power-user layout. | Do not confuse the separately maintained Columns UI component with the default interface or import its extension architecture into Phase 1. |

## Recommended direction

Interaction references complement product screenshots. NN/g's
[data-table task analysis](https://www.nngroup.com/articles/data-tables/) separates
finding, comparing, inspecting and acting on records. Its
[card guidance](https://www.nngroup.com/articles/cards-component/) describes the
space/scanning tradeoff of cards, and
[progressive disclosure](https://www.nngroup.com/articles/progressive-disclosure/)
supports making secondary complexity available on request. These are general
usability references, not an experiment on Daisy's listeners. Our inference is
to use covers for recognition and tables for track comparison, with advanced
controls in deliberate contextual panels.

[Microsoft navigation guidance](https://learn.microsoft.com/en-us/windows/apps/design/controls/navigationview)
provides a platform reference for expanded, compact and overlay navigation.
Borrow the behavior principle rather than its WinUI implementation or exact
breakpoints; Daisy remains cross-platform egui.

Three directions were considered. A pure album canvas is attractive for browsing
but poor for comparing many tracks. A fully populated technical workbench
maximizes simultaneous information but increases clutter and reduces readable
type. The recommended balance keeps artwork-led Home/album context and a precise
Library table within the same restrained shell. These are design judgments,
not results of a timed usability experiment.

Build a refined desktop music workspace. Its memorable detail is a coherent
relationship between cover artwork, selection and the persistent playing track,
not a trademark color or decorative effect. Neutral graphite is the foundation.
Music supplies most of the color. A small selectable accent identifies actions
and focus, with text/icons or structural changes also identifying state.

Home uses a modest album shelf and listening context. Library uses a proper
song/album/artist/playlist workspace, with a table where comparison is useful.
Playlist uses a compact cover/title header above tracks. These are distinct
compositions inside one shell, not repeated cards with different headings.

Keep navigation short and stable. Pins/folders remain subordinate to page
navigation. Global search and page-local filtering have distinct purposes.
The player maintains track identity, transport, seek and output access. Queue
and Lyrics are optional inspector views and close completely. Narrow desktop
windows keep music and transport usable; an inspector may become an explicit
overlay rather than squeezing the table into unreadable columns.

Use the shipped Inter weights and Lucide vocabulary first. Assess Geist with
native script fallback, metrics and real captures before replacing typography.
Use modest 6–10px radii for controls and restrained separation between surfaces.
No cream/gold default, Spotify-green dependency, glass wall, neon frame, marketing
hero, fake audio badge or invented feature description inside the product.

## Themes and density are independent

Propose Dark, Light and OLED base palettes, plus System preference in native
implementation. Accents are a separate choice: blue, teal, coral or neutral.
Each base/accent pair needs separate text, action-label and focus checks. Light
uses darker accents rather than transplanting a bright dark-theme accent onto
white. Optional artwork accents must be normalized and contrast-tested, never
change meaning or silently enable processing. Existing custom-theme files and
settings remain backward compatible. Mica/vibrancy is deferred until platform
behavior and readability can be verified.

Repository evidence already includes custom JSON palettes, bundled desktop
presets, a last-usable-theme cache and Linux Omarchy integration through
`fastframe-theme`. This redesign extends those capabilities; it must not remove
them or pretend themes are being invented from scratch. Current System/Light/Dark
choices and separate compact-sidebar/track-list settings are compatibility
inputs, not disposable legacy settings.

Compact/default/comfortable rows target 38/46/54 logical pixels. Density changes
row rhythm, not the entire information architecture. Settings must preserve the
old serialized choices and defaults when extended; no identity migration is
part of this redesign.

## Validation before applying the design

Review Home, Library and Playlist at 760x800, 1280x800, 1440x900 and the primary
2560x1440 display, with inspector open/closed. Inspect all base themes, accents,
densities, long titles, selection/current-track distinction, focus, keyboard
operation, loading, empty and error states. Record visual captures, not just
automated geometry. Use [WCAG 2.2](https://www.w3.org/TR/WCAG22/) as a useful
contrast/focus/target-size design reference; passing sampled pairs is not a
complete conformance claim for a native application. Focus Appearance is an AAA
criterion, distinct from the AA focus-visible and minimum-target requirements.

After the concrete Figma/token direction is reviewed, implement the semantic
theme layer, primitives and shell around existing Actions. Keep source changes
separate from authentication, queue algorithms, player engine and platform code.
Then validate native captures and existing backend contracts. Browser prototypes
cannot establish egui rendering quality or native performance.

## Remaining evidence

An independent primary-source comparison supported the shell, theme and table
recommendations. [Roon Queue](https://help.roonlabs.com/portal/en/kb/articles/the-queue)
adds concrete evidence for persistent access, current-track orientation and
contextual item controls. Its queue history and transfer semantics are reference
material, not permission to change Daisy's existing queue contract.

Direct desktop visual evidence for Plexamp is still outstanding; the inspected
mobile image is used only for artwork/personality evidence. Its
[September 2026 release notes](https://forums.plex.tv/t/plexamp-v4-50-3-ready-or-not/942338)
document a platform migration; this is not a recommendation to introduce Tauri
or a browser engine into Daisy. No
interviews, task-timing study or competitor installation was performed. Daisy subsequently selected Spotiphile and authorized continuation after the
replacement review. Native implementation is in progress; no Phase 1 checkbox
is completed merely because this research document exists.

The replacement composition and native restyle were subsequently rejected by
Daisy. Research observations remain inputs, but the resulting layout failed the
visual gate. [The second reset](PROFESSIONAL_DESIGN_REVIEW.md) records that gap.
