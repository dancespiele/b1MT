#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Translations {
    pub home: String,
    pub tokenomics: String,
    pub buy: String,
    pub stake: String,
    pub road_map: String,
    pub community: String,
    pub description: String,
    pub contract: String,
    pub launch: String,
    pub governance: String,
    pub kennel_token_pool: String,
    pub web: String,
    pub swap_web: String,
    pub automata_token: String,
    pub cross_swap: String,
    pub governance_web: String,
}

#[derive(Deserialize)]
pub struct Lang {
    pub en: Translations,
    pub es: Translations,
}
