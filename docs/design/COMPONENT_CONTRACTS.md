# Phase 1 component contracts, proposed

These contracts guide the design review. No component below is claimed to be
implemented in the native client or published as a Figma component.

| Component | States and behavior | Native boundary |
| --- | --- | --- |
| Navigation item / rail | Default, hover, pressed, focus, selected; visible label or accessible name in rail; current page remains identifiable without color alone | Existing navigation Actions and Page router |
| Button / icon button | Default, hover, pressed, focus, disabled, loading; unavailable action has a reason; reuse shipped Lucide vectors | Existing theme widgets and Actions |
| Search field | Idle, typing, focus, loading, empty results, failed request; preserve current cancellation/routing behavior | Existing Search Actions / backend |
| Track table / row | Density 38/46/54; stable columns, intentional long-text truncation, keyboard focus distinct from selection/current track; context menus retain current semantics | Existing collection/library/queue modules |
| Player bar | Persistent metadata, transport, seek and device/volume controls; signed-out/unavailable/connecting states; narrow layout keeps essential controls visible | Existing player Actions; never dispatch network work during drawing |
| Inspector | One active tab, fully closable, resizable at desktop; narrow overlay with Escape/close and focus return; remember compatible layout state | Existing queue/lyrics panels and their mutual exclusion |
| Loading / empty / error | Distinct labels; cached content stays usable when current backend allows it; retry only through existing request behavior; no color-only status | Existing page loading/error state |
| Theme / density controls | Dark/light/OLED and compact/default/comfortable specimens; system choice follows native preference; preserve old settings defaults | Theme layer plus backward-compatible settings fields after approval |

Queue 2.0, new shuffle algorithms, command palette, new metadata/provider model,
Signal Path claims and DSP changes belong to later phases. A design label does
not authorize those implementations. Preserve current shortcuts and feature
access, including Winamp and MilkDrop, while replacing shell presentation.

Review at 760x800, 1280x800 and 1440x900. Check inspector open/closed, rail/full
sidebar, all densities, dark/light/OLED, long names, keyboard-only navigation,
loading/empty/error, and reduced motion. Review typography and spacing in real
egui captures after implementation; HTML/Figma cannot validate native rendering.
