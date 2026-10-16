# projectM identifier bounds

This candidate selects the projectM-only portion of Spotifast commit
`6a60c305e6fe1e571f0e7aef184396e0c54c026d`, authored by Carmine Paolino.
It does not port that commit's Librespot changes.

The wrapper moves from dennisgr7/projectm-rs `1ef6df1c20247a3a5800c625e46262330f93bbd6`
to maintainer-owned crmne/projectm-rs `93852ff7299e5a63ed1315437593fc70915c0863`.
That single-commit comparison changes only the native submodule remote and pin.
The native library moves from `98101f56feea576f8e240061c457541abc797aa1` to
`ffb109857b9d31d71eef7df44f6e34b5a788e556`: one tokenizer length check before
copying an identifier into its 256-byte buffer. Rust APIs, features and build.rs
remain unchanged, preserving Windows generator and ARM handling and the Nix
libdir substitution.

## Validation

An independent source review found this port separable from the larger playback
migration. Cargo's locked graph accepts the new source. The lock diff changes
only projectm-sys's source; unrelated tempfile resolution churn was removed.

The C++ regression in `tests/native/projectm-tokenizer.cpp` was compiled with
MSVC against the actual fetched patched HLSLTokenizer.cpp and Engine.cpp, without
copying dependency implementation into this repository. It passed: a 255-byte
identifier is accepted, 256 and 4096 bytes are rejected to end-of-stream, and a
normal declaration retains its token sequence. This is a manually run boundary
test, not yet a CI job or a MilkDrop rendering test.

To repeat, set parserRoot to the fetched dependency's
`projectm-sys/libprojectM/vendor/hlslparser/src`, then run from a developer shell:

```powershell
cl /nologo /EHsc /std:c++17 /I $parserRoot tests/native/projectm-tokenizer.cpp "$parserRoot/HLSLTokenizer.cpp" "$parserRoot/Engine.cpp" /Fo.cache/ /Fe.cache/projectm-tokenizer-regression.exe
& .cache/projectm-tokenizer-regression.exe
```

Windows default/all-feature library suites passed 1,036/1,060 tests (two
inherited ignored each), plus all target suites. Formatting, all-feature
warnings-denied Clippy and the optimized build passed. Independent final source
review found no material findings.

[Hash discovery run](https://github.com/DaisyCatTs/SpotiPhile/actions/runs/38845414893)
calculated `sha256-90XLra1Vry6gx0iS1EdXl/9mWX9+4Tcr+Vq879Lg1z4=`.
Its lockfile SHA-256 matches the Git blob independently:
`2f94ea1ca1ae72a844ece5914bb26aba1b630322232f4170029ec5b239411ae7`.
The real hash is now applied. Discovery is not a successful package build.
Default warnings-denied Clippy, doctests and warnings-denied Rustdoc also passed.
Full Nix package and cross-platform CI and native MilkDrop rendering remain
pending. This candidate is not merge-ready.

## Attribution and licensing

The change does not modify license files. Retain projectm-sys's LGPL-3.0-or-later
notice, libprojectM's LGPL text, and the bundled HLSL parser's MIT attribution
(Unknown Worlds Entertainment, 2013–2014). Static linking obligations remain;
a version-pin change does not itself establish distribution compliance.
