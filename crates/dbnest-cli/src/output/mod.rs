mod human;
mod json;

pub use human::{print_instance_human, print_instances_human};
pub use json::{print_instance_json, print_instances_json};

use dbnest_core::{Instance, InstanceSummary};

use serde_json::json;

pub fn print_ok(json_mode: bool, action: &str, id: Option<&str>) {
    if json_mode {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "ok": true,
                "action": action,
                "id": id
            }))
            .unwrap()
        );
    } else {
        if let Some(id) = id {
            println!("{action} ok: {id}");
        } else {
            println!("{action} ok");
        }
    }
}

pub fn print_status(json_mode: bool, res: crate::cli::StatusResult) {
    if json_mode {
        match res {
            crate::cli::StatusResult::One(r) => {
                println!("{}", serde_json::to_string_pretty(&r).unwrap())
            }
            crate::cli::StatusResult::Many(v) => {
                println!("{}", serde_json::to_string_pretty(&v).unwrap())
            }
        }
    } else {
        match res {
            crate::cli::StatusResult::One(r) => print_status_human(&[r]),
            crate::cli::StatusResult::Many(v) => print_status_human(&v),
        }
    }
}

fn print_status_human(list: &[dbnest_core::InstanceStatusReport]) {
    if list.is_empty() {
        println!("No instances found.");
        return;
    }

    for r in list {
        println!("{}  {:8}  {:?}", r.id, r.engine.as_str(), r.status);
    }
}

pub fn print_instance(inst: &Instance, json: bool) {
    if json {
        print_instance_json(inst);
    } else {
        print_instance_human(inst);
    }
}

pub fn print_instances(list: &[InstanceSummary], json: bool) {
    if json {
        print_instances_json(list);
    } else {
        print_instances_human(list);
    }
}
