# Rustls TLS boundary patch

The lockfile selects Rustls 0.23.45 instead of 0.23.43. The only dependency
changes are that package's version and checksum. Cargo.toml, client identities,
permissions, credential formats, proxies and application version are unchanged.
The existing tempfile/getrandom selection is retained; `cargo tree --locked`
accepts the resulting graph.

## Reason and provenance

[RUSTSEC-2026-0285](https://rustsec.org/advisories/RUSTSEC-2026-0285.html) and the
[maintainer advisory](https://github.com/rustls/rustls/security/advisories/GHSA-2mjx-qc3c-rqvc)
identify affected versions 0.23.13 through 0.23.44, fixed in 0.23.45. Rustls
accepted certain TLS 1.3 handshake messages across encryption-level boundaries.
The transcript remains authenticated; the advisory does not support claiming
an attacker can alter or complete a handshake or that credentials were exposed.

This adapts upstream `68c453e1c9fc524bca8d13f12a4ae6cbd9ad6465`, retaining only
the Rustls update and deriving this fork's vendor hash independently. Source
adaptation commit: `b2e8796`. Upstream's unrelated lockfile resolution change and
its Nix vendor hash are not copied.

Original author: Carmine Paolino. Upstream coauthor: Chris
(`25006584+seichris@users.noreply.github.com`).

The inverse locked graph contains one Rustls package, reached through reqwest,
hyper-rustls, tokio-rustls, websockets, QUIC and Librespot. It therefore covers
the existing HTTP clients and playback session transport without a new direct
dependency or a Librespot API migration.

## Nix hash derivation

Audit branch `audit/rustls-vendor-hash` builds only this fork's fixed-output
dependency derivation. An expected hash mismatch is hash discovery, not a
successful package build. The helper checks that the mismatch belongs to
Spotifast's vendor derivation and matches the old configured hash, then records
the result together with the exact Cargo.lock SHA-256 in the job summary.
That helper exists only on the isolated audit branch and publishes no package.

The discovered hash must be applied to flake.nix and the unchanged required
`nix build .#default` job must pass before promotion. No supplier hash is reused
and no package check is skipped.

## Validation status

Primary advisories and checksum were verified. Locked dependency graph inspection
passed. Windows default/all-feature suites passed 1,036/1,060 library tests
(two inherited ignored each), plus all target suites. Formatting, both
warnings-denied Clippy configurations, doctests, warnings-denied Rustdoc and
the optimized build passed. The release reports `spotifast 0.12.0`.
Nix hash discovery and complete cross-platform validation are pending.
No authenticated account operation or actual user credential is used by tests.

The first isolated hash-discovery run reused the old fixed-output archive and
failed its deliberate expectation of a mismatch. This was not package
validation. The follow-up audit commit sets a dummy hash on that audit branch
only, forcing dependency retrieval and hash calculation. The product branch
retains its real hash until the result is known.
