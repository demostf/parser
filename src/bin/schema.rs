use schemars::schema_for;
use tf_demo_parser::demo::packet::Packet;

fn main() {
    let schema = schema_for!(Packet);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
