pub use snowflake::*;

#[macro_export]
macro_rules! struct_id_impl {
  ($id_ident: ident, $struct_ident:ident, $field_ident:ident) => {
    impl $struct_ident {
      pub fn uid(&self) -> $id_ident {
        self.$field_ident
      }
    }
  };
}

#[macro_export]
macro_rules! struct_id {
  ($T: ident) => {
    use snowflake::ProcessUniqueId;
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
    pub struct $T(pub ProcessUniqueId);

    impl $T {
      pub fn new() -> $T {
        $T(ProcessUniqueId::new())
      }
    }
  };
}
