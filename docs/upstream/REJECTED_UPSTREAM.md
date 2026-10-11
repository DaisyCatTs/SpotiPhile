# Rejected or deferred upstream changes

No individual commit has been rejected. Of the 17 commits after the baseline,
ten fixes are ported on this stable candidate; seven are not yet included here.
Four of those seven are validated on the development branch (minimized close,
Mac GPU handling, Turkish wording and Ukrainian). Three still need individual
review (shuffle hover, Blend attribution and Linux custom chrome).

Wholesale merges are rejected as the integration strategy. Review reliability,
security, performance and platform fixes first. Preserve product functionality
and distinguish behavioral fixes from changes requiring visual approval.
DRM circumvention, fake Spotify Lossless and browser-engine replacement remain
outside product scope. These are policies, not commit-level rejection claims.
