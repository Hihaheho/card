pub struct LanguageCode(u16);

pub struct I18nString {
    en: String,
    map: HashMap<LanguageCode, String>,
}
