// uuid
fn main2() {
    // let _v4 = uuid::Uuid::new_v4().to_string();
    // let _v4 = uuid::Uuid::new_v4().to_string();
    // println!("{}", "123".to_string());
    // println!("{}", _v4);
    // println!("{}", "123".to_string());
    // println!("{}", "123".to_string());

    let _get_uuid = get_uuid();
    println!("{}",_get_uuid)
}

fn get_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}
