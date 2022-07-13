use std::time::Duration;

use kap::{Kap, KapValue, Keygroup};

#[tokio::main]
async fn main() {
  println!("[info]: Keep typing numbers within 2seconds. Maxium of 10 digits.");

  loop {
    Kap::new()
      .within(
        Duration::from_secs_f32(1.0),
        &[KapValue::from_group(Keygroup::Number)],
        10,
        true,
      )
      .await
      .task(|record| println!("[info]: Pressed {:?}", record))
      .catch(|record| println!("[info]: Catch, pressed keys {:?}", record))
      .done();
  }
}
