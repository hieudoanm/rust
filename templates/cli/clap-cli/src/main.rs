use uuid::Uuid;

fn main() {
  let uuid = &Uuid::new_v4().to_string();
  println!("{}", uuid)
}
