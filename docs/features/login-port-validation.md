# Web API callback reservation

Focused port of Spotifast commit
`98b37e7d333c76193f80967ab301b089de010c60`, authored by Sam Boland
(upstream #768). Web API sign-in reserves its loopback callback listener before
invalidating a grant, changing authorization state, offering a sign-in link or
opening the browser. A port conflict reports an actionable failure and leaves
the existing authorization generation alone. Retrying can proceed after the
port is released.

The production redirect remains `127.0.0.1:8989/login`; permissions, registered
client identities, protected storage and PKCE/state checks are unchanged.
Playback authorization remains a separate flow. This port does not claim to
reserve its callback before opening the browser.

The upstream backend regression assumed fixed port 8989 was free. The adapted
test instead reserves an ephemeral port and exercises the same private
grant-start helper used by production. Both shared and personal failure routes
are checked, including repeated attempts, no browser-wait event, unchanged
credential generations and restore flags, and untouched authorization state.
Production still selects its original registered grants and ports.

Independent source review confirmed the ordering and retained cancellation,
account and late-result guards, and cleared the test adaptation. The listener
regression releases an occupied temporary port, retries and receives a synthetic
callback with the expected state, without contacting Spotify or opening a browser.

## Validation

The initial upstream port passed Windows x64 default/all-feature library suites
(976/1,000 tests, two existing ignored each), all target suites, both
warnings-denied Clippy configurations, formatting, doctests, warnings-denied
Rustdoc and an optimized build/version smoke. Jekyll also built successfully.
The adapted revision repeated both complete test suites (976/1,000 library
tests, two existing ignored each), both Clippy configurations and the optimized
build/version smoke successfully. Four unchanged launcher tests passed under
WSL using the existing LF-normalized packaging copy. The final documentation
site build passed as well. Formatting and final diff checks passed.

No authenticated sign-in or macOS/Linux runtime session was exercised. Tests use
dummy flows and isolated credential stores; existing local Spotify credentials
and login settings are not changed. No new grant, dependency, persistence schema,
browser engine or layout change is introduced.

## PR #13 review corrections

The initial direct grant-start regression missed the production personal-app
configuration command, which revoked its old grant before reserving the callback.
Replacement now preflights HTTP configuration, the Client ID and the listener
before revocation, stored deletion or API replacement. It hands that same listener
to authorization without a release/rebind gap. If an existing authorization owns
the port, its cancellation completes first and the queued replacement repeats
preflight. Explicit removal or sign-out discards the queued replacement.

The application saves only the backend-accepted Client ID, even while the field
contains an edited draft or another preference is saved. A failed preflight
restores the existing configuration in both the field and settings file. The
verified personal grant indicator remains unchanged on failure.

Listener creation uses Tokio TcpSocket and restores SO_REUSEADDR only on Unix,
matching [Tokio's listener defaults](https://docs.rs/tokio/1.53.1/tokio/net/struct.TcpSocket.html).
Windows retains address reuse disabled. A Unix-only regression completes three
synthetic callbacks with immediate rebinding while also checking that a live
listener remains exclusive. That regression requires Unix CI; it was not run on
Windows.

Targeted Windows tests passed for the production configuration command with an
occupied ephemeral port, including a queued shared authorization, retained dummy
stored grant/token provider, credential generation, API account/state and restore
flags. Owned-listener cancellation/removal and draft persistence/rollback tests
passed. Independent source review cleared the corrections. Corrected Windows
validation passed: 1,033 default and 1,057 all-feature library tests (two existing
ignored tests each), all target suites, formatting, both warnings-denied Clippy
configurations, doctests, warnings-denied Rustdoc, optimized build and
`spotifast 0.12.0` version smoke. Jekyll and final diff checks also passed.
Unix runtime validation remains assigned to required CI before promotion.
