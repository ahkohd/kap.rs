use device_query::Keycode;
use kap::{Kap, KapValue};

#[tokio::main]
async fn main() {
  println!("[info]: Basic example, press A");

  Kap::new()
    .until(KapValue::from(Keycode::A))
    .await
    .task(|| println!("[info]: Pressed A"))
    .finally(|| {
      println!("[info]: Done");
    });
}
