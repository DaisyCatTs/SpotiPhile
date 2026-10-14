# Rejected or deferred upstream changes

No individual commit has been rejected. The earlier reviewed snapshot had 17
commits after the baseline; sixteen are ported here. A fresh fetch found twenty
newer commits, whose review is tracked separately. Token Debug hardening is also
ported, making seventeen accepted of thirty-seven in the latest inventory.

- Deferred: `14db97c82263400d180100135d631c9885245ce2` (Linux custom title bar).
It changes window chrome and needs the separate visual review while UI redesign
is paused. This is a deferred port, not rejection of Linux support or upstream
functionality. Main promotion of this candidate still requires protected CI.

Wholesale merges are rejected as the integration strategy. Review reliability,
security, performance and platform fixes first. Preserve product functionality
and distinguish behavioral fixes from changes requiring visual approval.
DRM circumvention, fake Spotify Lossless and browser-engine replacement remain
outside product scope. These are policies, not commit-level rejection claims.

Machine-readable entries use `Deferred: SHA` or `Rejected: SHA` on the decision
line. Mentioning an upstream SHA alone does not reject it. New pending commits
belong in the review inventory, not this decision ledger.
