# Ordered queue playlist saving

The development branch preserves the current item and loaded upcoming queue
occurrences when saving a private playlist. Previously it silently deduplicated
the sequence. Existing playlist additions also sent all selected items in one
request, exceeding Spotify's limit for selections above 100.

The API client now sends sequential batches of at most 100 items. Explicit
insertion positions advance by each preceding batch, so insertion and append
both retain order. Empty additions perform no authentication or network request.
The final successful batch supplies the resulting playlist snapshot ID.

Contract: [Spotify Add Items to Playlist](https://developer.spotify.com/documentation/web-api/reference/add-items-to-playlist).
No new dependency, grant, credential handling or persistence format is introduced.

## Validation

- Loopback HTTP regression covers 205 ordered repeated items, append and position
  7 insertion, returned snapshot ID, and a second-batch 403 stopping further writes.
- Application regression checks that the playing item and repeated upcoming
  occurrences remain in their exact order.
- Empty-input regression checks that no authentication is required.
- Windows x64 full checks passed: 1,031 default and 1,055 all-feature library
  tests (two existing ignored tests), all binary/integration suites, both
  warnings-denied Clippy configurations, formatting, doctests and warnings-denied
  Rustdoc. Optimized build and `spotifast 0.12.0` version smoke passed.
- Four unchanged launcher tests passed in the established LF-normalized WSL
  copy. Jekyll built the documentation in 9.872 seconds.
- These checks cover the combined development branch. A focused main promotion
  must pass fresh protected platform CI; macOS/Linux runtime is not claimed.

## Limits

Authenticated creation on Spotify has not been exercised for this revision.
The master-plan checkbox remains in progress. Only loaded queue rows are saved,
not unseen context pages. A partial failure leaves earlier accepted batches in
the created playlist and displays the existing error. There is no rollback or
automatic replay of accepted batches. Concurrent edits by another client are
outside this snapshot's guarantees.
