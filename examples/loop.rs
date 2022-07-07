use std::time::Duration;

use device_query::Keycode;
use kap::{Kap, KapValue};

#[tokio::main]
async fn main() {
  println!("[info]: Basic example, press A");

  loop {
    Kap::new()
      .until(KapValue::from(Keycode::A))
      .await
      .task(|| println!("[info]: Pressed A"))
      .sleep(Duration::from_millis(500))
      .await
      .finally(|| {
        println!("[info]: Done");
      });
  }
}
