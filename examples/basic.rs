use device_query::Keycode;
use kap::{Kap, KapValue};

#[tokio::main]
async fn main() {
  println!("[info]: Basic example, press A twice");

  Kap::new()
    .until(&[KapValue::from(Keycode::A)], 2)
    .await
    .task(|record| println!("[info]: Pressed {:?}", record))
    .finally(|_| {
      println!("[info]: Done");
    });
}
