mod human;
mod json;

pub use human::{print_instance_human, print_instances_human};
pub use json::{print_instance_json, print_instances_json};

use dbnest_core::{Instance, InstanceSummary};

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
