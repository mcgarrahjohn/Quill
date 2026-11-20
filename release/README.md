# Release files

`Quill_0.1.0_aarch64.dmg` is the macOS installer for Apple Silicon Macs, built on this Mac (unsigned, so macOS shows an "unidentified developer" warning the first time; right-click the app and choose Open).

These files are **not meant to be committed to git** (installers are large and change every release). They belong on a GitHub Release page: the landing page's download buttons point at
`https://github.com/mcgarrahjohn/quill/releases/download/v<version>/<file name>`.

The Intel Mac, Windows and Linux installers are built by the GitHub "Release" workflow (see QUILL_TODO.md section 8), which uploads every file to the release for you. You can also upload this `.dmg` by hand to a release tagged `v0.1.0`.
