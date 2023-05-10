use crate::app::providers::i18n::{Translation, TK};

use common_macros::hash_map;

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
      TK::Hello => "你好",
      TK::World => "世界",
      TK::AnticipatedInscriptions => "⭕️ 預期的銘文",
  )
}
