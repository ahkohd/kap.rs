use kap::{Kap, KapValue, Keygroup};
use std::time::Duration;

#[tokio::main]
async fn main() {
  loop {
    println!("[info]: Press any key");

    Kap::new()
      .any()
      .await
      .task(|keycodes| println!("[info]: You pressed: {:?}", keycodes.last()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| println!("[info]: Press any alphabetic key"))
      .until(&[KapValue::from_group(Keygroup::Alphabet)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| println!("[info]: Press any number"))
      .until(&[KapValue::from_group(Keygroup::Number)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| println!("[info]: Press any function key"))
      .until(&[KapValue::from_group(Keygroup::FunctionKey)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| {
        println!("[info]: Press any modifier key i.e (Ctrl, Alt, Shift, Meta/Command/Windows)")
      })
      .until(&[KapValue::from_group(Keygroup::ModifierKey)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| {
        println!("[info]: Press any navigation key");
      })
      .until(&[KapValue::from_group(Keygroup::NavigationKey)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| {
        println!("[info]: Press any symbol i.e (`, -, =, [, ], \\, /, ;, ')");
      })
      .until(&[KapValue::from_group(Keygroup::Symbol)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .task(|_| {
        println!("[info]: Press any numeric key");
      })
      .until(&[KapValue::from_group(Keygroup::NumericKey)])
      .await
      .task(|keycodes| println!("[info]: You pressed {:?}", keycodes.last().unwrap()))
      .sleep(Duration::from_millis(500))
      .await
      .finally(|_| {
        println!("[info]: Done");
      });
  }
}
