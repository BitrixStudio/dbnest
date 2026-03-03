mod cli;
mod output;

use clap::Parser;
use cli::{Cmd, Root};
use output::{print_instance, print_instances};

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> dbnest_core::Result<()> {
    let root = Root::parse();

    match root.cmd {
        Cmd::Up(args) => {
            let inst = args.run()?;
            print_instance(&inst, root.json);
        }
        Cmd::Ls(args) => {
            let list = args.run()?;
            print_instances(&list, root.json);
        }
        Cmd::Stop(args) => {
            let id = args.id.clone();
            args.run()?;
            output::print_ok(root.json, "stop", Some(&id));
        }
        Cmd::Rm(args) => {
            let id = args.id.clone();
            args.run()?;
            output::print_ok(root.json, "rm", Some(&id));
        }
        Cmd::Apply(args) => {
            let id = args.id.clone();
            args.run()?;
            output::print_ok(root.json, "apply", Some(&id));
        }
        Cmd::Plan(args) => {
            let plan = args.run()?;
            if root.json {
                println!("{}", serde_json::to_string_pretty(&plan).unwrap());
            } else {
                for s in plan.statements {
                    println!("{s}\n");
                }
            }
        }
        Cmd::Status(args) => {
            let res = args.run()?; // StatusResult, not Result
            output::print_status(root.json, res);
        }
    }

    Ok(())
}
