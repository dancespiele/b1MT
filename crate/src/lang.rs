#[derive(Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Translations {
    pub home: String,
    pub tokenomics: String,
    pub use_cases: String,
    pub buy: String,
    pub stake: String,
    pub road_map: String,
    pub community: String,
    pub price: String,
    pub market_cap: String,
    pub description: String,
    pub community_project: String,
    pub contract: String,
    pub launch: String,
    pub governance: String,
    pub kennel_token_pool: String,
    pub web: String,
    pub swap_web: String,
    pub partner_app: String,
    pub automata_token: String,
    pub cross_swap: String,
    pub governance_web: String,
    pub partner_with_1mt: String,
    pub partner_with_1mt_description: String,
    pub vegas_casino_description: String,
    pub the_kennel_token_description: String,
    pub slogan: String,
    pub slogan_description: String,
    pub moneytime: String,
}

#[derive(Deserialize)]
pub struct Lang {
    pub en: Translations,
    pub es: Translations,
}
