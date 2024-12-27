# Rust Key-Value Store with Compression

This is a simple key-value store written in Rust using the `rocksdb` database and supporting compression via the `snap` crate. The project provides basic commands for setting, getting, deleting, and batching key-value pairs, along with automatic compression and decompression for stored values.

## Features

- **Set**: Store a key-value pair with compression.
- **Get**: Retrieve a value by its key, automatically decompressing it.
- **Delete**: Remove a key-value pair.
- **BatchSet**: Store multiple key-value pairs in a single batch operation, with compression.

## Dependencies

- `clap`: A simple command-line argument parser.
- `rocksdb`: Embedded database to store key-value pairs.
- `snap`: Library for compression and decompression of data using the Snappy algorithm.

## Usage

The program supports the following subcommands:

### 1. Set

Store a key-value pair:

```bash
cargo run -- set <key> <value>
```

Example:

```bash
cargo run -- set key1 value1
```

### 2. Get

Retrieve the value of a given key:

```bash
cargo run -- get <key>
```

Example:

```bash
cargo run -- get key1
```

### 3. Delete

Remove a key-value pair:

```bash
cargo run -- delete <key>
```

Example:

```bash
cargo run -- delete key1
```

### 4. Batch Set

Store multiple key-value pairs in a single batch:

```bash
cargo run -- batch-set <key1> <value1> <key2> <value2> ...
```

Example:

```bash
cargo run -- batch-set key1 value1 key2 value2
```

## Structure

The project consists of the following key components:

- `KVStore`: The main struct that interacts with the RocksDB database.
  - Provides methods for setting, getting, deleting, and batch setting data.
  - Automatically compresses data before storing it and decompresses it when retrieving.
- `Cli`: A command-line interface that uses `clap` to parse arguments and execute corresponding actions.

## Installation

To run this project locally, follow these steps:

1. Ensure that you have Rust installed on your system. If not, install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Clone the repository:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

3. Build and run the project:

   ```bash
   cargo build
   cargo run -- <command> <args>
   ```
