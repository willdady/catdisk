# catdisk

A simple file cataloging utility which recursively walks a directory and writes to an SQLite database.

## Setup and installation

1. Install the Rust toolchain with [rustup](https://rustup.rs/)
2. Run `cargo install --path .` to install `catdisk`
3. Confirm installation by running `catdisk --help`

## Usage

```bash
catdisk [FILE] [PATH]
```

e.g.

```bash
catdisk my-files.db /Volume/MyFiles/
```

Note, `catdisk` will overwrite the database file if it exists.

## Database schema

The output SQLite database will have a table named `files` with the following columns:

| Column Name | Data Type | Description                                |
| ----------- | --------- | ------------------------------------------ |
| path        | TEXT      | Path to a file relative to the source path |
| ext         | TEXT      | The file extension (if it exists)          |
| bytes       | INTEGER   | Size of the file in bytes                  |
| created     | INTEGER   | File creation timestamp                    |
| modified    | INTEGER   | File modification timestamp                |

Use the database file with your favourite SQLite client.
The following examples use the `sqlite3` command line client.

### Count files

```bash
sqlite3 my-files.db 'SELECT count(*) FROM files;'
```

### Find the largest file by size in Megabytes

```bash
sqlite3 my-files.db 'SELECT path, CAST(bytes as FLOAT) / 1000000 as megabytes FROM files ORDER BY bytes DESC LIMIT 1;'
```

### Select file paths by extension

```bash
sqlite3 my-files.db 'SELECT path FROM files WHERE ext = "md"'
```
