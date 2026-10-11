# 0004: Spotiphile display identity and compatible appearance preferences

Status: Accepted for display identity and settings compatibility. Date: 2026-10-08.
Phase 1 implementation remains in progress; the visual composition was rejected.

## Context

Daisy chose Spotiphile after rejecting the original cream/gold design and then
instructed continuation after the replacement review. Existing profiles and
custom themes must survive UI changes. Older Spotifast readers reject unknown
ThemeChoice enum variants and can discard the entire settings object.

## Decision

Use Spotiphile in the main window, login and About presentation. Keep Cargo,
application/profile IDs, keychain namespaces, Connect names, updater coordinates
and upstream attribution unchanged pending a separately designed migration.
This does not establish a new released package or change Spotify authentication.

Represent OLED using existing `theme: "dark"` plus optional
`daisy_theme_variant: "oled"`. Accent and row density are independent optional
fields. Unknown or malformed values in these new fields resolve to None without
rejecting unrelated valid settings. Existing custom palettes retain precedence
and exact colors. Built-in palettes currently use the proposed neutral direction;
their precise visual tokens remain subject to the replacement design review.
Unset density retains legacy row dimensions; an explicit choice sets 38/46/54
logical pixels. UI controls emit Actions; deferred atomic persistence remains
owned by App and Settings.

## Alternatives and consequences

A new ThemeChoice::Oled variant was rejected because an older binary could fail
to parse the whole profile. A package/profile rename now was rejected because
it requires coordinated migration and release/update ownership work. Ignored
new fields preserve old-reader compatibility, but an older binary may drop them
when it rewrites settings. Existing compact-list preferences remain independent.

## Validation

Settings tests cover representative old-reader round trips and unknown/malformed
new fields preserving volume and custom theme selection. App tests exercise
OLED/accent/density Actions, custom palette precedence and saved preferences.
Native captures and check results are recorded in
[Phase 1 progress](../design/NATIVE_PROGRESS.md). This ADR does not mark the
whole phase complete or certify native screen-reader support on every platform.
