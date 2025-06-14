#[derive(Serialize, Deserialize)]
pub struct Monster {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: Vec<String>,
    pub size: String,
    #[serde(rename = "type")]
    pub monster_type: String,
    pub alignment: String,
    pub ac: String,
    pub hp: String,
    pub speed: String,
    pub str: String,
    pub dex: String,
    pub con: String,
    pub int: String,
    pub wis: String,
    pub cha: String,
    #[serde(rename = "save")]
    pub save_1: Option<String>,
    pub skill: Option<Vec<String>>,
    pub passive: Option<String>,
    pub languages: Option<String>,
    pub cr: String,
    pub resist: Option<Vec<String>>,
    pub immune: Option<String>,
    pub vulnerable: Option<String>,
    #[serde(rename = "conditionImmune")]
    pub condition_immune: Option<String>,
    pub senses: Option<String>,
    #[serde(rename = "trait")]
    pub monster_trait: Option<Vec<MonsterTrait>>,
    pub action: Option<Vec<MonsterAction>>,
    pub description: Vec<String>,
    pub environment: Option<String>,
    pub legendary: Option<Vec<MonsterLegendary>>,
    pub slots: Option<Vec<String>>,
    pub spells: Option<String>,
    pub reaction: Option<Vec<Reaction>>,
    pub ancestry: Option<String>,
    pub sortname: Option<String>,
    pub sense: Option<String>,
    pub attack: Option<Vec<MonsterAttack>>,
    pub exp: Option<String>,
    pub npc: Option<String>,
    pub conditionimmune: Option<String>,
    pub experience: Option<Experience>,
    pub loot: Option<String>,
    pub ancenstry: Option<String>,
    pub monster: Option<MonsterMonster>,
    pub text: Option<String>,
    pub environemnt: Option<String>,
    pub imune: Option<String>,
    pub skills: Option<String>,
    #[serde(rename = "Save")]
    pub save: Option<String>,
    pub resistance: Option<String>,
    pub immunity: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterTrait {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: String,
    pub text: String,
    pub attack: Option<Vec<String>>,
    pub recharge: Option<String>,
    pub roll: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterAction {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: Option<String>,
    pub text: Option<Vec<String>>,
    pub attack: Option<Vec<String>>,
    pub recharge: Option<String>,
    pub action: Option<Vec<MonsterActionAction>>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterActionAction {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: String,
    pub text: Vec<String>,
    pub attack: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterLegendary {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: String,
    pub recharge: Option<Vec<String>>,
    pub text: String,
    pub attack: Option<Vec<String>>,
    pub action: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: Option<String>,
    pub text: Option<String>,
    pub recharge: Option<String>,
    pub attack: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterAttack {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: Option<String>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Experience {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonster {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub name: String,
    pub size: String,
    #[serde(rename = "type")]
    pub monster_type: String,
    pub alignment: String,
    pub ac: String,
    pub hp: String,
    pub speed: String,
    pub str: String,
    pub dex: String,
    pub con: String,
    pub int: String,
    pub wis: String,
    pub cha: String,
    pub save: String,
    pub skill: String,
    pub passive: String,
    pub languages: String,
    pub cr: String,
    pub exp: String,
    pub resist: MonsterMonsterResist,
    pub immune: MonsterMonsterImmune,
    pub vulnerable: MonsterMonsterVulnerable,
    #[serde(rename = "conditionImmune")]
    pub condition_immune: MonsterMonsterConditionImmune,
    pub senses: String,
    #[serde(rename = "trait")]
    pub monster_trait: Vec<MonsterMonsterTrait>,
    pub action: Vec<MonsterMonsterAction>,
    pub legendary: Vec<MonsterMonsterLegendary>,
    pub spells: MonsterMonsterSpells,
    pub slots: MonsterMonsterSlots,
    pub description: String,
    pub environment: MonsterMonsterEnvironment,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterResist {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterImmune {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterVulnerable {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterConditionImmune {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterTrait {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: String,
    pub text: String,
    pub recharge: Option<String>,
    pub attack: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterAction {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub name: String,
    pub text: String,
    pub attack: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterLegendary {
    #[serde(rename = "$text")]
    pub text_content: Option<String>,
    pub text: String,
    pub name: Option<String>,
    pub attack: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterSpells {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterSlots {
}

#[derive(Serialize, Deserialize)]
pub struct MonsterMonsterEnvironment {
}

