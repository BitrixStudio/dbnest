use dbnest_core::{Instance, InstanceSummary};

pub fn print_instance_json(inst: &Instance) {
    println!("{}", serde_json::to_string_pretty(inst).unwrap());
}

pub fn print_instances_json(list: &[InstanceSummary]) {
    println!("{}", serde_json::to_string_pretty(list).unwrap());
}
