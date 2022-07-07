use device_query::Keycode;

fn normalize_keycode(key: Keycode) -> String {
  let mut key = key.to_string().replace("LControl", "Control");
  key = key.replace("RControl", "Control");
  key = key.replace("LShift", "Shift");
  key = key.replace("RShift", "Shift");
  key = key.replace("LAlt", "Alt");
  key = key.replace("RAlt", "Alt");
  key
}

pub fn assert_keycode_equals(first: Vec<Keycode>, second: Vec<Keycode>) -> bool {
  if first.len() != second.len() {
    return false;
  }

  let first = first
    .iter()
    .map(|key| normalize_keycode(*key))
    .collect::<Vec<String>>();
  let second = second
    .iter()
    .map(|key| normalize_keycode(*key))
    .collect::<Vec<String>>();

  first.iter().all(|key| second.contains(key))
}
