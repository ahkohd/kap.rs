pub mod utils;

pub use device_query::Keycode;
use device_query::{DeviceQuery, DeviceState};
use tokio::time::{self, Duration, Instant};

/// An enum representing the state of Kap
///
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

#[derive(Debug, Default)]
pub struct KapConfig {
  loop_delay: Duration,
}

#[derive(Debug)]
pub struct Kap {
  state: KapState,
  keycodes: KeycodesRecord,
  config: KapConfig,
}

impl Default for Kap {
  fn default() -> Self {
    Kap {
      state: KapState::default(),
      keycodes: KeycodesRecord::default(),
      config: KapConfig {
        loop_delay: Duration::from_millis(50),
      },
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

  pub fn config(&mut self, config: KapConfig) {
    self.config = config;
  }

  pub async fn sleep(&mut self, duration: Duration) -> &mut Self {
    self.state = KapState::Next;
    time::sleep(duration).await;
    self
  }

  pub async fn until(&mut self, values: &[KapValue]) -> &mut Self {
    if let KapState::Done = self.state {
      return self;
    }

    let device_state = DeviceState::new();
    let mut interval = time::interval(self.config.loop_delay);

    loop {
      interval.tick().await;
      let keys = device_state.get_keys();
      if values.iter().any(|value| value.test(&keys)) {
        self.state = KapState::Next;
        self.record_value(keys);
        return self;
      }
    }
  }

  pub async fn any(&mut self) -> &mut Self {
    if let KapState::Done = self.state {
      return self;
    }

    let device_state = DeviceState::new();
    let mut interval = time::interval(self.config.loop_delay);

    loop {
      interval.tick().await;
      let keys = device_state.get_keys();

      if !keys.is_empty() {
        self.state = KapState::Next;
        self.record_value(keys);
        break;
      }
    }

    self
  }

  pub async fn within(&mut self, timeout: Duration, others: &[KapValue]) -> &mut Self {
    if let KapState::Done = self.state {
      return self;
    }

    let device_state = DeviceState::new();
    let mut interval = time::interval(self.config.loop_delay);
    let start = Instant::now();

    let check = loop {
      interval.tick().await;

      if start.elapsed() >= timeout {
        break false;
      }

      let keys = device_state.get_keys();

      if others.iter().any(|other| other.test(&keys)) {
        break true;
      }
    };

    self.state = if check {
      KapState::Next
    } else {
      KapState::Fail
    };

    self.record_value(device_state.get_keys());
    self
  }

  pub async fn after(&mut self, timeout: Duration, others: &[KapValue]) -> &mut Self {
    self.sleep(timeout).await;
    self.until(others).await;
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
