# Spotiphile visual reset: composition before styling

Date: 2026-10-08. Status: current prototype and native composition rejected by Daisy.
The Spotiphile name and request for themes remain. No visual merge approval exists.

Resumed 2026-10-09. The initial `professional-shell.html` attempt also failed
internal review: oversized now-playing hero, nested cards, duplicated queue
content, product-facing development copy and clipped content at 1280×800.
Its name was not evidence of visual quality. Replacement work must remove those
defects before presenting a new review.

## What went wrong

The native work mostly retained Spotifast's composition. Changing palette,
font sizes and row heights did not establish a distinct product design. The
1280px native Home has a navigation/library/filter toolbar, eight shortcut
slabs, artwork shelves, status controls and queue all competing in one viewport.
The inspector duplicates Queue across two rows. The player contains many useful
controls but little distinction between everyday and occasional actions.

The HTML review moved further, but its attractive artwork and many theme options
were not evidence that the app's composition was resolved. Its review controls
also distracted from assessing the music workspace. Passing geometry checks
only proved specific layout properties. It did not establish professional
visual quality. The research-to-implementation step was insufficient.

## Research decisions grounded in inspected desktop references

- [Apple Music's Windows column browser](https://support.apple.com/en-ca/guide/music-windows/musd4821f613/windows)
  demonstrates stable alignment and clear category-to-track relationships in the
  inspected screenshot. Adopt stable scan lines and contextual filtering. Do not
  stack collection filters in every region of the screen.
- [Roon's Albums documentation](https://help.roonlabs.com/portal/en/kb/articles/albums)
  and its inspected album screen give artwork, record identity and contextual
  actions a defined place. Adopt clear ownership of identity and primary action.
  Its editorial serif identity, metadata capabilities and audio format badges do
  not automatically fit a native Spotify client.
- [MusicBee's official imagery](https://getmusicbee.com/) demonstrates that album
  browsing, navigation and now-playing context can share a desktop workspace.
  Its older promotional screenshot is not a current-version usability study.
  Do not copy its full toolbar density or multiple simultaneous information areas.
- [Audirvana Studio](https://audirvana.com/audirvana-studio/) documents a redesigned
  desktop interface and unified navigation. Adopt separation of library browsing
  from optional technical controls. The observed technical panel is not a
  template for the primary music screen or permission for unsupported audio claims.

Primary images and source uncertainty are preserved in research-image-evidence.json.
These observations are design inputs, not proof that one product is objectively best.

## Replacement constraints

One clear main task per screen. Home prioritizes playable music and artwork;
Library prioritizes sorting and scanning; a Playlist prioritizes the track list.
Navigation owns routes and saved collections. Page controls own sorting/filtering.
Transport owns current playback. An optional inspector owns upcoming music or
lyrics, with one selection row, not stacked duplicate navigation.

Use a coherent neutral surface hierarchy, restrained accent and Inter's existing
native weights. Use rectangular selected navigation and quiet separators;
reserve filled emphasis for the primary playback action. Use meaningful album
art, not decorative gradients or nested panels. Themes change tokens rather than
rearranging the application. Long titles, keyboard focus and error content must
have designed treatment.

Do not carry another composition into native Rust before a full-screen review
establishes that it improves the existing result. Preserve current functional
work as uncommitted development evidence; do not merge or silently erase it.

## Review gates

- Home's first viewport reads as music, not a control dashboard or roadmap.
- One visible global search and one inspector tab row.
- Collections and track metadata align consistently and have distinct roles.
- Common actions are obvious; occasional controls are contextual and discoverable.
- Artwork crop, type scale, spacing and control shape are consistent across views.
- Narrow layouts keep navigation and transport usable without crushing the page.
- Dark/light captures use matching content and geometry; themes do not conceal
  layout defects. Geometry/accessibility checks complement visual review.
- Before/after review must include the complete workspace. Attractive individual
  cards and green test results do not satisfy the visual gate.

## Further rejection and refinement, 2026-10-09

Daisy explicitly rejected the 25:2 composition proposal as unfinished and visually
weak. The subsequent "alr" did not approve its appearance. The 27 geometry checks
remain historical evidence of layout properties only. Figma frame 25:2 and its
captures must not be treated as the canonical approved design.

The next revision targets typography, navigation hierarchy, surface contrast,
track-table finishing, artwork presentation and transport controls. This is
visual proposal work; it does not authorize another broad native restyle.

[Microsoft typography guidance](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/typography)
was checked on 2026-10-09. Its regular body text, restrained semibold emphasis,
sentence casing and explicit size/line-height relationships support a calmer
scan hierarchy. Applying those principles to Inter is our cross-platform design
judgment, not a claim of adopting the Windows typeface or its implementation.
Apple's typography page returned a JavaScript-only shell and was not used as
substantive evidence in this pass.

The official use_figma API was retried read-only on 2026-10-09 and returned the
Starter-plan tool limit before execution. The working local TalkToFigma channel
remains the route for supported primitive operations. No purchase is required
to continue the visual proposal and QA.

## Refined proposal, 2026-10-10

Current source: `professional-shell.html`; evidence: `refinement-validation.json`
and `refinement-comparison.html`. Root rejected an intermediate oversized focus
block and decorative waveform, then removed both. The final Home keeps bounded
artwork and three recent tracks in the first viewport at the tested default sizes.
The player owns transport, with one inspector opener. Navigation selection uses
solid restrained fill; playlist thumbnails replace anonymous repeated disc icons.

Independent review identified missing narrow thumbnail names, visual-only
selection with lost keyboard modifiers, and inaccessible narrow filters. Fixes
exclude thumbnail spans when finding names, add named native selection buttons
with pressed state, preserve keyboard modifiers and keep filters visible. All
focused checks and 27 page/theme/viewport cases passed in headless Chromium.
No new page JavaScript errors were recorded. This does not establish full
accessibility certification or satisfaction of Daisy's visual preference.

The latest complete screen is a local HTML design artifact. Existing Figma frame
25:2 is explicitly marked rejected. No new Figma variables/components, native
restyle, package migration, Phase 2 work, commit, push or merge is included.

A targeted follow-up review found the new selection focus ring could be clipped
by the first cell. Selection controls now use an inset focus outline.
