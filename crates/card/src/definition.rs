use i18n::I18nString;

pub struct CardDefinition {
    pub name: I18nString,
    pub description: I18nString,
    pub hooks: Vec<CardHook>,
}
