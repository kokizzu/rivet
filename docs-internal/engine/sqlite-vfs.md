# SQLite VFS

Rules for the native SQLite VFS implementation.

## Package boundaries

- RivetKit SQLite is native-only. VFS and query execution live in `rivetkit-rust/packages/rivetkit-sqlite/`, core owns lifecycle, and NAPI only marshals JS types.
- RivetKit TypeScript SQLite is exposed through `@rivetkit/rivetkit-napi`, but runtime behavior stays in `rivetkit-rust/packages/rivetkit-sqlite/` and `rivetkit-core`.
- The Rust KV-backed SQLite implementation lives in `rivetkit-rust/packages/rivetkit-sqlite/src/`. When changing its on-disk or KV layout, update the internal data-channel spec in the same change.
- The VFS uses a 4 KiB chunk layout for page storage. PRAGMAs are pinned at open: `journal_mode = DELETE`, `locking_mode = EXCLUSIVE`, `auto_vacuum = NONE`. Source: `rivetkit-rust/packages/rivetkit-sqlite/src/vfs.rs`.

## VFS implementation notes

- SQLite VFS aux-file create/open paths mutate `BTreeMap` state under one write lock with `entry(...).or_insert_with(...)`. Avoid read-then-write upgrade patterns.
- SQLite VFS v2 storage keys use literal ASCII path segments under the `0x02` subspace prefix with big-endian numeric suffixes so `scan_prefix` and `BTreeMap` ordering stay numerically correct.
- SQLite v2 slow-path staging writes encoded LTX bytes directly under DELTA chunk keys. Do not expect `/STAGE` keys or a fixed one-chunk-per-page mapping in tests or recovery code.
