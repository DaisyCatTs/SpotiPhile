# Rejected or deferred upstream changes

No individual commit has been rejected. Of the 17 commits after the baseline,
sixteen fixes are ported on this focused candidate. The remaining upstream change,
`14db97c82263400d180100135d631c9885245ce2` (Linux custom title bar), is deferred.
It changes window chrome and needs the separate visual review while UI redesign
is paused. This is a deferred port, not rejection of Linux support or upstream
functionality. Main promotion of this candidate still requires protected CI.

Wholesale merges are rejected as the integration strategy. Review reliability,
security, performance and platform fixes first. Preserve product functionality
and distinguish behavioral fixes from changes requiring visual approval.
DRM circumvention, fake Spotify Lossless and browser-engine replacement remain
outside product scope. These are policies, not commit-level rejection claims.
