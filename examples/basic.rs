use device_query::Keycode;
use kap::{Kap, KapValue};

#[tokio::main]
async fn main() {
  println!("[info]: Basic example, press A");

  Kap::new()
    .until(&[KapValue::from(Keycode::A)])
    .await
    .task(|_| println!("[info]: Pressed A"))
    .finally(|_| {
      println!("[info]: Done");
    });
}
