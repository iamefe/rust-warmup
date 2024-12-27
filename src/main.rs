use clap::{Parser, Subcommand};
use rocksdb::{Options, WriteBatch, DB};
use snap::raw::{Decoder, Encoder};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Metadata not showing in CLI
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
    BatchSet { pairs: Vec<String> },
}

struct KVStore {
    db: DB,
}

impl KVStore {
    fn new(path: &Path) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        Ok(Self {
            db: DB::open(&opts, path)?,
        })
    }

    fn set(&self, key: &str, value: &str) -> Result<(), rocksdb::Error> {
        let compressed = self.compress(value.as_bytes());
        self.db.put(key.as_bytes(), compressed)
    }

    fn get(&self, key: &str) -> Result<Option<String>, rocksdb::Error> {
        match self.db.get(key.as_bytes())? {
            Some(value) => {
                let decompressed = self.decompress(&value);

                Ok(Some(String::from_utf8_lossy(&decompressed).to_string()))
                // String::from_utf8_lossy(&decompressed) converts the byte slice into a Cow<str> (a Clone-on-Write string type).
                // If the byte slice is valid UTF-8, it creates a string without any changes.
                // If the byte slice contains invalid UTF-8, it replaces invalid sequences with the Unicode replacement character (�).
                // This ensures the operation does not panic.
            }
            None => Ok(None),
        }
    }

    fn delete(&self, key: &str) -> Result<(), rocksdb::Error> {
        self.db.delete(key.as_bytes())
    }

    fn batch_set(&self, pairs: Vec<(&str, &str)>) -> Result<(), rocksdb::Error> {
        let mut batch = WriteBatch::default();
        for (key, value) in pairs {
            let compressed = self.compress(value.as_bytes());
            batch.put(key.as_bytes(), compressed);
        }
        self.db.write(batch)
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.compress_vec(data).unwrap_or(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Vec<u8> {
        let mut decoder = Decoder::new();
        decoder.decompress_vec(data).unwrap_or(data.to_vec())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = KVStore::new(Path::new("./kv-store-db"))?;
    let cli = Cli::parse();
    // Cli::parse() returns a Cli instance such as:
    /*
    Cli {
        command: Commands::Set {
            key: "key1".to_string(),
            value: "value1".to_string(),
        }
    }
    */
    match cli.command {
        Commands::Set { key, value } => {
            store.set(&key, &value)?;
            println!("Set {} = {}", key, value);
        }
        Commands::Get { key } => match store.get(&key)? {
            Some(value) => println!("{} = {}", key, value),
            None => println!("Key '{}' not found", key),
        },
        Commands::Delete { key } => {
            store.delete(&key)?;
            println!("Deleted key: {}", key);
        }
        Commands::BatchSet { pairs } => {
            let pairs: Vec<(&str, &str)> = pairs
                .chunks(2)
                .map(|chunk| (chunk[0].as_str(), chunk[1].as_str()))
                .collect();
            store.batch_set(pairs)?;
            println!("Batch set completed");
        }
    }
    Ok(())
}
