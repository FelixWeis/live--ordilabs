use crate::app::providers::i18n::{Translation, TK};

use common_macros::hash_map;

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
      TK::Hello => "हैलो",
      TK::World => "वर्ल्ड",
      TK::AnticipatedInscriptions => "⭕️ अपेक्षित अभिलेख",
  )
}
