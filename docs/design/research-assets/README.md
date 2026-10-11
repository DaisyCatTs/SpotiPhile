# Review artwork

These three original images were generated on 2026-10-08 with the built-in
image-generation tool for the Phase 1 design review and inspected before use.
They illustrate fictional music, not the official artwork of any named artist
or the song playing in Daisy's authenticated client.

- `shoreline-cover.png`: blue-hour industrial shoreline, small figure and reflected lights.
- `glass-cover.png`: sculptural teal sea glass on dark rock.
- `architecture-cover.png`: cropped terracotta architecture, deep shadows and sun.

Each image is a 1254x1254 PNG, copied intact from the generated output. There are
no image edits or external asset requests. The consuming HTML may use ordinary
cover sizing/cropping. Remaining code-drawn cover graphics in the prototype are
original review illustrations.

The native application continues to use its existing Spotify image pipeline.
These images are review assets, not new runtime dependencies or replacement
album artwork. Competitor screenshots used for research stay in ignored scratch
storage and are recorded by URL/hash in `research-image-evidence.json`.

`InterVariable.ttf` and its accompanying `Inter-LICENSE.txt` are copied from the
existing locked `fastframe-fonts` dependency (fastframe checkout revision
`23d87e0`). The review can therefore render the same font family locally without
an external font service. This does not change the native font dependency.
