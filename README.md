# gawk 

`gawk` is a CLI tool designed to check a developer's git repositories and report if they are in a clean state. The use case envisioned is to utilize `gawk` prior to migrating to a different physical machine or prior to a context switch in projects, where the tool reports on the status of a developer's git repositories at one time.

This is also designed to be a learning experience to whet my whistle with Rust, and is a re-implementation of my original Java-based project [`jitter`](https://github.com/baskinomics/jitter).