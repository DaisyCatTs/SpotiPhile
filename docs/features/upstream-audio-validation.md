# Upstream audio reliability with SpotiPhile queue controls

Validated on 2026-10-11, Windows x64, feature/queue-controls. Exact ports of
Spotifast #726/#731 are commits 52f29af and ed30344. Original authorship is
preserved. No queue, appearance, authentication or dependency source changed.

The writer waits for consumed queued sound with a 50 ms timeout. Failed outputs
are maintained/reopened instead of immediately stopping playback, and a full
queue stalled for two seconds is reopened. Suspension gaps reset the stall
clock. Failure to reopen remains a reported error. Existing queue bounds,
seek/track transitions, volume and pre-volume visualizer boundaries remain.

## Executed checks

- Formatting passed.
- Default full suite: 1,008 library tests passed, one existing credential-store
  test ignored; binary and integration suites passed.
- All-features full suite: 1,031 library tests passed with the same ignored
  test; binary, integration and native queue/UI suites passed.
- Both Clippy configurations passed with warnings denied.
- Documentation tests and all-features Rustdoc with warnings denied passed.
- Optimized Windows build passed in 2m 18s; version smoke returned spotifast 0.12.0.
- Upstream inventory smoke showed three accepted source commits and 14 pending.
- Final source diff reviewed: the two ports only change src/sink.rs. The native
  queue edit journal, selection and context protections remain unchanged.

The independently validated promotion branch passed 28 sink tests and full
946/969-library-test suites, both Clippy checks and a 2m 23s release build.
PR #2 is stacked on protected PR #1. Neither promotion includes the unapproved
appearance work. Main promotion awaits required checks and actual GitHub review.

No authenticated playback, real Bluetooth/default-device switch, sleep/wake,
Linux/macOS runtime check or new CPU/memory benchmark is claimed. Upstream CPU
measurements are not SpotiPhile measurements. Cross-platform CI is a separate
record. The existing ignored native credential-store test is not weakened.
