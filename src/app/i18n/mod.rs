use crate::app::providers::i18n::Locale;

use super::providers::Translation;

mod de;
mod en;
mod hi;
mod zn;

#[allow(dead_code)]
pub fn locale_data(l: Locale) -> Translation {
  match l {
    Locale::HI => hi::translation(),
    Locale::DE => de::translation(),
    Locale::ZN => zn::translation(),
    Locale::EN => en::translation(),
  }
}
