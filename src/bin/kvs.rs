extern crate structopt;

use kvs::KvStore;
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

fn main() {
    let mut kvs = KvStore::new();
    let opt: KvsCommand = KvsCommand::from_args();

    if opt.version_flag {
        println!(env!("CARGO_PKG_VERSION"));
        return;
    }
    let cmd = opt.cmd.unwrap();
    unimplemented_exit();
    match cmd {
        KvsSubCommands::Get { key } => {
            let value = kvs.get(key).unwrap_or_default();
            println!("{}", value);
        }
        KvsSubCommands::Set { key, value } => kvs.set(key, value),
        KvsSubCommands::Rm { key } => kvs.remove(key)
    }
}


fn unimplemented_exit() {
    std::io::stderr().write_all(b"unimplemented").unwrap();
    std::process::exit(1)
}
