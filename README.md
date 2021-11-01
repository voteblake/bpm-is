BPM Is
====

A static website that allows a user to tap along with a beat, displaying the song's calculated tempo in beats per minute (BPM).

Hosted at https://bpm.is

Build
----

Can be built with standard `cargo` but works best when used in conjunction with [`trunk`](https://trunkrs.dev/) to build the full website instead of just the `wasm` binary.

Use the `trunk serve` command to run a local instance of the page which watches for file changes and reloads,
or use `trunk build` (optionally with `--release` flags) to populate the `dist/` directory with build artifacts. You can serve those artifacts locally with tools
like [`miniserve`](https://github.com/svenstaro/miniserve) or deploy to the static hosting platform of your choice.
