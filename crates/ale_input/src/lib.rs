use bitflags::bitflags;

#[derive(Debug)]
pub enum Input {
  Key(Key, Scancode, Action, Modifier),
  Char(char),
  MouseMotion {
    rel_x: f32,
    rel_y: f32,
    abs_x: f32,
    abs_y: f32,
  },
  MouseButton(MouseButton, Action, Modifier),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
  Space,
  Apostrophe,
  Comma,
  Minus,
  Period,
  Slash,
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
  Semicolon,
  Equal,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  LeftBracket,
  Backslash,
  RightBracket,
  GraveAccent,
  World1,
  World2,
  Escape,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  Right,
  Left,
  Down,
  Up,
  PageUp,
  PageDown,
  Home,
  End,
  CapsLock,
  ScrollLock,
  NumLock,
  PrintScreen,
  Pause,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  F13,
  F14,
  F15,
  F16,
  F17,
  F18,
  F19,
  F20,
  F21,
  F22,
  F23,
  F24,
  F25,
  Kp0,
  Kp1,
  Kp2,
  Kp3,
  Kp4,
  Kp5,
  Kp6,
  Kp7,
  Kp8,
  Kp9,
  KpDecimal,
  KpDivide,
  KpMultiply,
  KpSubtract,
  KpAdd,
  KpEnter,
  KpEqual,
  LeftShift,
  LeftControl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightControl,
  RightAlt,
  RightSuper,
  Menu,
  Unknown,
}

pub type Scancode = i32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Action {
  Release,
  Press,
  Repeat,
}

bitflags! {
  pub struct Modifier : u32 {
    const SHIFT     = 0b00000001;
    const CONTROL   = 0b00000010;
    const ALT       = 0b00000100;
    const SUPER     = 0b00001000;
    const CAPSLOCK  = 0b00010000;
    const NUMLOCK   = 0b00100000;
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
  ButtonLeft,   // Button1
  ButtonRight,  // Button2
  ButtonMiddle, // Button3
  Button4,
  Button5,
  Button6,
  Button7,
  Button8,
}
