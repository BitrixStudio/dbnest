mod cli;
mod output;

use clap::Parser;
use cli::{Cmd, Root};
use output::{print_instance, print_instances};

fn main() {
    let root = Root::parse();

    let result = match root.cmd {
        Cmd::Up(args) => {
            let inst = args.run().unwrap();
            print_instance(&inst, root.json);
            Ok(())
        }
        Cmd::Ls(args) => {
            let list = args.run().unwrap();
            print_instances(&list, root.json);
            Ok(())
        }
        Cmd::Stop(args) => args.run(),
        Cmd::Rm(args) => args.run(),
    };

    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}