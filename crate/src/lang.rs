#[derive(Deserialize, Clone, Default, PartialEq)]
pub struct Translations {
    pub description: String,
    pub contract: String,
}

#[derive(Deserialize)]
pub struct Lang {
    pub en: Translations,
    pub es: Translations,
}
