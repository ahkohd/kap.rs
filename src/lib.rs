pub mod utils;

use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc,
};

pub use device_query::Keycode;
use device_query::{DeviceEvents, DeviceQuery, DeviceState};
use tokio::time::{self, Duration, Instant};

/// An enum representing the state of Kap
///
#[derive(Debug)]
enum KapState {
  Fail,
  Next,
  Done,
}

impl Default for KapState {
  fn default() -> Self {
    KapState::Next
  }
}

/// An enum that groups keycodes
///
pub enum Keygroup {
  Number,
  Alphabet,
  Symbol,
  ModifierKey,
  FunctionKey,
  NavigationKey,
  NumericKey,
}

impl Keygroup {
  /// Returns an array of keycodes that belong to this group

  pub fn get_keycodes(self) -> &'static [Keycode] {
    match self {
      Keygroup::Number => &[
        Keycode::Key0,
        Keycode::Key1,
        Keycode::Key2,
        Keycode::Key3,
        Keycode::Key4,
        Keycode::Key5,
        Keycode::Key6,
        Keycode::Key7,
        Keycode::Key8,
        Keycode::Key9,
      ],
      Keygroup::Alphabet => &[
        Keycode::A,
        Keycode::B,
        Keycode::C,
        Keycode::D,
        Keycode::E,
        Keycode::F,
        Keycode::G,
        Keycode::H,
        Keycode::I,
        Keycode::J,
        Keycode::K,
        Keycode::L,
        Keycode::M,
        Keycode::N,
        Keycode::O,
        Keycode::P,
        Keycode::Q,
        Keycode::R,
        Keycode::S,
        Keycode::T,
        Keycode::U,
        Keycode::V,
        Keycode::W,
        Keycode::X,
        Keycode::Y,
        Keycode::Z,
      ],
      Keygroup::FunctionKey => &[
        Keycode::F1,
        Keycode::F2,
        Keycode::F3,
        Keycode::F4,
        Keycode::F5,
        Keycode::F6,
        Keycode::F7,
        Keycode::F8,
        Keycode::F9,
        Keycode::F10,
        Keycode::F11,
        Keycode::F12,
      ],
      Keygroup::ModifierKey => &[
        Keycode::LControl,
        Keycode::LShift,
        Keycode::LAlt,
        Keycode::Meta,
        Keycode::RControl,
        Keycode::RShift,
        Keycode::RAlt,
      ],
      Keygroup::NavigationKey => &[
        Keycode::Up,
        Keycode::Down,
        Keycode::Left,
        Keycode::Right,
        Keycode::Home,
        Keycode::End,
        Keycode::PageUp,
        Keycode::PageDown,
      ],
      Keygroup::NumericKey => &[
        Keycode::Numpad0,
        Keycode::Numpad1,
        Keycode::Numpad2,
        Keycode::Numpad3,
        Keycode::Numpad4,
        Keycode::Numpad5,
        Keycode::Numpad6,
        Keycode::Numpad7,
        Keycode::Numpad8,
        Keycode::Numpad9,
        Keycode::NumpadAdd,
        Keycode::NumpadSubtract,
        Keycode::NumpadMultiply,
        Keycode::NumpadDivide,
      ],
      Keygroup::Symbol => &[
        Keycode::Grave,
        Keycode::Minus,
        Keycode::Equal,
        Keycode::LeftBracket,
        Keycode::RightBracket,
        Keycode::BackSlash,
        Keycode::Slash,
        Keycode::Semicolon,
        Keycode::Apostrophe,
        Keycode::Comma,
        Keycode::Dot,
        Keycode::Slash,
      ],
    }
  }
}

/// A struct that represents a keystroke.
/// It can be a keycode, combination of keycodes or a group of keycodes

pub struct KapValue {
  keys: Option<Vec<Keycode>>,
  is_group: bool,
}

impl KapValue {
  /// Create a KapValue from Keycode

  pub fn from(key: Keycode) -> Self {
    KapValue {
      keys: Some(vec![key]),
      is_group: false,
    }
  }

  /// Create a KapValue from Vec<Keycode>

  pub fn from_keys(keys: Vec<Keycode>) -> Self {
    KapValue {
      keys: Some(keys),
      is_group: false,
    }
  }

  /// Create a KapValue from Keygroup
  /// This will store all keycodes in the group in the KapValue

  pub fn from_group(group: Keygroup) -> Self {
    KapValue {
      keys: Some(group.get_keycodes().to_vec()),
      is_group: true,
    }
  }

  /// Create a KapValue from Keygroups

  pub fn from_groups(groups: Vec<Keygroup>) -> Self {
    let mut keys = Vec::new();
    for group in groups {
      keys.extend_from_slice(group.get_keycodes());
    }
    KapValue {
      keys: Some(keys),
      is_group: true,
    }
  }

  /// Tests given &[Keycode] against the stored &[Keycode] in a KapValue
  /// There are two test modes:
  /// 1. Test if the given Keycodes are equal to the stored get_keycode
  /// 2. Test if the given Keycodes are a subset of the stored get_keycodes

  fn test(&self, others: &[Keycode]) -> bool {
    let keys = self.keys.as_ref().unwrap();

    if self.is_group {
      return utils::assert_keycode_equals_any(keys, others);
    }

    utils::assert_keycode_equals(keys, others)
  }
}

type KeycodesRecord = Vec<Vec<Keycode>>;

#[derive(Debug)]
pub struct Kap {
  state: KapState,
  keycodes: KeycodesRecord,
  is_keydown: Arc<AtomicBool>,
}

impl Default for Kap {
  fn default() -> Self {
    Kap {
      state: KapState::default(),
      keycodes: KeycodesRecord::default(),
      is_keydown: Arc::new(AtomicBool::new(false)),
    }
  }
}

impl Kap {
  pub fn new() -> Self {
    Kap::default()
  }

  fn record_value(&mut self, value: Vec<Keycode>) {
    self.keycodes.push(value);
  }

  pub async fn sleep(&mut self, duration: Duration) -> &mut Self {
    self.state = KapState::Next;
    time::sleep(duration).await;
    self
  }

  async fn on_keydown<F>(&mut self, device_state: &DeviceState, mut test: F)
  where
    F: FnMut(&mut Self) -> bool,
  {
    let is_keydown_clone = self.is_keydown.clone();
    let _guard = device_state.on_key_down(move |_| is_keydown_clone.store(true, Ordering::Relaxed));
    let mut interval = time::interval(Duration::from_millis(10));

    loop {
      interval.tick().await;

      if test(self) {
        break;
      }

      self.is_keydown.store(false, Ordering::Relaxed);
    }

    self.is_keydown.store(false, Ordering::Relaxed);
  }

  fn is_keydown(&self) -> bool {
    self.is_keydown.load(Ordering::Relaxed)
  }

  pub async fn until(&mut self, values: &[KapValue], repeat: usize) -> &mut Self {
    if let KapState::Next = self.state {
      let device_state = DeviceState::new();
      let mut captured_keys: Vec<Keycode> = vec![];

      self
        .on_keydown(&device_state, |kap| {
          if kap.is_keydown() {
            let keys = device_state.get_keys();

            if values.iter().any(|value| value.test(&keys)) {
              captured_keys.append(&mut device_state.get_keys());

              if captured_keys.len() >= repeat {
                kap.state = KapState::Next;
                kap.record_value(captured_keys.clone());
                return true;
              }
            }
          }

          false
        })
        .await;
    }

    self
  }

  pub async fn any(&mut self) -> &mut Self {
    if let KapState::Next = self.state {
      let device_state = DeviceState::new();

      self
        .on_keydown(&device_state, |kap| {
          if kap.is_keydown() {
            let keys = device_state.get_keys();

            if !keys.is_empty() {
              kap.state = KapState::Next;
              kap.record_value(keys);
              return true;
            }
          }

          false
        })
        .await;
    }

    self
  }

  pub async fn within(
    &mut self,
    timeout: Duration,
    others: &[KapValue],
    max_repeat: usize,
    debounce: bool,
  ) -> &mut Self {
    if let KapState::Next = self.state {
      let device_state = DeviceState::new();
      let mut start = Instant::now();
      let mut captured_keys: Vec<Keycode> = vec![];

      // Capture keypress of characters in the set until the length
      // of the captured keypresses is greater than or equals
      // to max_repeat or the timeout is elapsed.
      // (debounce -> true) Restart timer on every valid  keypress.
      //
      // If user types a character that's not in set, abort!
      //
      self
        .on_keydown(&device_state, |kap| {
          if start.elapsed() >= timeout || captured_keys.len() >= max_repeat {
            kap.state = if !captured_keys.is_empty() {
              KapState::Next
            } else {
              KapState::Fail
            };

            if !captured_keys.is_empty() {
              kap.record_value(captured_keys.clone());
            }

            return true;
          }

          if kap.is_keydown() {
            let mut keys = device_state.get_keys();

            if others.iter().any(|other| other.test(&keys)) {
              captured_keys.append(&mut keys);

              if debounce {
                start = Instant::now();
              }
            } else {
              kap.state = KapState::Fail;
              captured_keys.append(&mut keys);
              kap.record_value(captured_keys.clone());
              return true;
            }
          }

          false
        })
        .await;
    }

    self
  }

  pub async fn after(
    &mut self,
    timeout: Duration,
    others: &[KapValue],
    repeat: usize,
  ) -> &mut Self {
    self.sleep(timeout).await;
    self.until(others, repeat).await;
    self
  }

  pub fn task<T>(&mut self, callback: T) -> &mut Self
  where
    T: Fn(KeycodesRecord),
  {
    if let KapState::Next = self.state {
      callback(self.keycodes.clone());
    }

    self
  }

  pub fn task_async<F>(&mut self, task: F) -> &mut Self
  where
    F: Fn(KeycodesRecord) + Send + 'static,
  {
    if let KapState::Next = self.state {
      let keycodes = self.keycodes.clone();
      tokio::spawn(async move {
        task(keycodes.clone());
      });
    }

    self
  }

  pub fn catch<T>(&mut self, callback: T) -> &mut Self
  where
    T: Fn(KeycodesRecord),
  {
    if let KapState::Fail = self.state {
      callback(self.keycodes.clone());
    }

    self
  }

  pub fn done(&mut self) {
    self.state = KapState::Done;
  }

  pub fn finally<T>(&mut self, callback: T)
  where
    T: Fn(KeycodesRecord),
  {
    callback(self.keycodes.clone());
    self.done();
  }
}
