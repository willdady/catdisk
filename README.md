# catdisk

File cataloging utility which recursively walks a directory and writes to a SQLite database.

## Setup and installation

1. Install the rust toolchain with [rustup](https://rustup.rs/)
2. Build with `cargo build`

## Usage

```bash
catdisk [FILE] [PATH]
```

e.g.

```bash
catdisk my-files.db /Volume/MyFiles/
```

## Database schema

TODO