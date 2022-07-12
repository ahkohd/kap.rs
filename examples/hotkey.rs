use kap::{Kap, KapValue, Keycode};
use std::time::Duration;

#[tokio::main]
async fn main() {
  loop {
    println!("Press Cmd+Shift+A");

    Kap::new()
      .until(&[KapValue::from_keys(vec![
        Keycode::Meta,
        Keycode::LShift,
        Keycode::A,
      ])])
      .await
      .task(|_| {
        clear();
        println!("Nice! Then press <Esc>");
      })
      .within(
        Duration::from_secs_f32(1.0),
        &[KapValue::from(Keycode::Escape)],
      )
      .await
      .task(|_| {
        clear();
        println!("Let's fucking go!");
      })
      .catch(|_| println!("Too slow, try again!"))
      .sleep(Duration::from_secs_f32(1.0))
      .await
      .finally(|_| clear());
  }
}

fn clear() {
  print!("{esc}c", esc = 27 as char);
}
