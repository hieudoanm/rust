use argh::FromArgs;

#[derive(FromArgs)]
/// Hello Options
struct HelloOptions {
  /// name
  #[argh(option)]
  name: String,
}

fn main() {
  let options: HelloOptions = argh::from_env();
  println!("Hello, {}!", options.name);
}
