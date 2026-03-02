use uuid::Uuid;

pub fn new_instance_id() -> String {
    // 8 chars from a uuid is fine for local dev instances
    let u = Uuid::new_v4().simple().to_string();
    u[..8].to_string()
}
