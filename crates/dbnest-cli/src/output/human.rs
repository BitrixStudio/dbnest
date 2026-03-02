use dbnest_core::{Instance, InstanceSummary};

pub fn print_instance_human(inst: &Instance) {
    println!("Instance ID:     {}", inst.id);
    println!("Engine:          {}", inst.engine.as_str());
    println!("Backend:         {:?}", inst.backend);
    println!();
    println!("DATABASE_URL={}", inst.connection.database_url);
}

pub fn print_instances_human(list: &[InstanceSummary]) {
    if list.is_empty() {
        println!("No instances found.");
        return;
    }

    for s in list {
        println!(
            "{}  {:8}  {:?}  {:?}  {}",
            s.id,
            s.engine.as_str(),
            s.backend,
            s.status,
            s.database_url
        );
    }
}
