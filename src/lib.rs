mod utils;

pub use device_query::Keycode;
use device_query::{DeviceQuery, DeviceState};
use futures::Future;
use tokio::time::{self, Duration, Instant};

#[derive(Debug)]
enum KapState {
  Start,
  Fail,
  Next,
  Done,
}

impl Default for KapState {
  fn default() -> Self {
    KapState::Start
  }
}

pub struct KapValue {
  keys: Option<Vec<Keycode>>,
}

impl KapValue {
  /// Create a KapValue from KeyCode

  pub fn from(key: Keycode) -> Self {
    KapValue {
      keys: Some(vec![key]),
    }
  }

  /// Create a KapValue from Vec<Keycode>

  pub fn from_keys(keys: Vec<Keycode>) -> Self {
    KapValue { keys: Some(keys) }
  }

  fn test(&self, keys: &[Keycode]) -> bool {
    utils::assert_keycode_equals(self.keys.clone().unwrap(), keys.to_vec())
  }

  fn get_keycodes(self) -> Vec<Keycode> {
    self.keys.unwrap()
  }
}

#[derive(Debug, Default)]
pub struct Kap {
  state: KapState,
  keycodes: Vec<Vec<Keycode>>,
}

impl Kap {
  pub fn new() -> Self {
    Kap::default()
  }

  fn record_value(&mut self, value: KapValue) {
    self.keycodes.push(value.get_keycodes());
  }

  pub async fn sleep(&mut self, duration: Duration) -> &mut Self {
    self.state = KapState::Next;
    time::sleep(duration).await;
    self
  }

  pub async fn until(&mut self, value: KapValue) -> &mut Self {
    if let KapState::Done = self.state {
      return self;
    }

    let device_state = DeviceState::new();
    let mut interval = time::interval(Duration::from_millis(50));

    loop {
      interval.tick().await;
      let keys = device_state.get_keys();
      if value.test(&keys) {
        self.state = KapState::Next;
        self.record_value(value);
        return self;
      }
    }
  }

  pub async fn within(&mut self, timeout: Duration, other: KapValue) -> &mut Self {
    if let KapState::Done = self.state {
      return self;
    }

    let device_state = DeviceState::new();
    let mut interval = time::interval(Duration::from_millis(50));
    let start = Instant::now();

    let check = loop {
      interval.tick().await;

      if start.elapsed() >= timeout {
        break false;
      }

      let keys = device_state.get_keys();

      if other.test(&keys) {
        break true;
      }
    };

    if check {
      self.state = KapState::Next;
      self.record_value(other);
    } else {
      self.state = KapState::Fail;
    }

    self
  }

  pub async fn after(&mut self, timeout: Duration, other: KapValue) -> &mut Self {
    self.sleep(timeout).await;
    self.until(other).await;
    self
  }

  pub fn task<T>(&mut self, callback: T) -> &mut Self
  where
    T: Fn(),
  {
    if let KapState::Next = self.state {
      callback();
    }

    self
  }

  pub fn task_async<F>(&mut self, task: F) -> &mut Self
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    if let KapState::Next = self.state {
      tokio::spawn(task);
    }

    self
  }

  pub fn catch<T>(&mut self, callback: T) -> &mut Self
  where
    T: Fn(),
  {
    if let KapState::Fail = self.state {
      callback();
    }

    self
  }

  pub fn done(&mut self) {
    self.state = KapState::Done;
  }

  pub fn finally<T>(&mut self, callback: T)
  where
    T: Fn(),
  {
    callback();
    self.done();
  }
}
