extern crate structopt;

use kvs::{Result, KvStore};
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "kvs")]
struct KvsCommand {
    #[structopt(short = "V")]
    version_flag: bool,
    #[structopt(subcommand)]
    cmd: Option<KvsSubCommands>,
}

#[derive(StructOpt)]
enum KvsSubCommands {
    #[structopt(name = "get")]
    Get {
        key: String
    },
    #[structopt(name = "set")]
    Set {
        key: String,
        value: String,
    },
    #[structopt(name = "rm")]
    Rm {
        key: String
    },
}

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let mut kvs = KvStore::open(current_dir.as_path())?;
    let opt: KvsCommand = KvsCommand::from_args();

    if opt.version_flag {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    match opt.cmd {
        Some(KvsSubCommands::Get { key }) => {
            let value = kvs.get(key)?
                .unwrap_or("Key not found".to_string());
            println!("{}", value);
        }
        Some(KvsSubCommands::Set { key, value }) => {
            let val = kvs.set(key, value);
            if val.is_err() {
                // println!("Key not found");
                println!("{}",val.err().unwrap());
            }
        }
        Some(KvsSubCommands::Rm { key }) => {
            let removed = kvs.remove(key);
            if removed.is_err() {
                println!("Key not found");
                std::process::exit(1);
            }
        }
        None => unimplemented_exit()
    }

    Ok(())
}


fn unimplemented_exit() {
    std::io::stderr().write_all(b"unimplemented\n").unwrap();
    std::process::exit(1)
}
