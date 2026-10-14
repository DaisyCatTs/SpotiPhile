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
