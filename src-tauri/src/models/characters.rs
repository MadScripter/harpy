use serde::{Serialize, Deserialize};

pub type CharacterResponse = Vec<CharacterElement>;

#[derive(Serialize, Deserialize)]
pub struct CharacterElement {
    pub account_id: String,
    pub character_id: String,
    pub character_name: CharacterName,
    pub body_type: i64,
    pub customization_options: CharacterCustomizationOptions,
    pub current_loadout: String,
    pub loadouts: Vec<Loadout>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterName {
    pub last: String,
    pub first: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharacterCustomizationOptions {
    pub eyes: AnimSet,
    pub skin_tone: AnimSet,
    pub head: AnimSet,
    pub voice: AnimSet,
    pub hair_style: HairStyle,
    pub anim_set: AnimSet,
    pub facial_hair: AnimSet,
}

#[derive(Serialize, Deserialize)]
pub struct AnimSet {
    pub id: String,
    pub variant: AnimSetVariant,
}

#[derive(Serialize, Deserialize)]
pub struct AnimSetVariant {
}

#[derive(Serialize, Deserialize)]
pub struct HairStyle {
    pub id: String,
    pub variant: HairStyleVariant,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HairStyleVariant {
    pub eyebrow_color: String,
    pub hair_base_color: String,
    pub hair_tip_color: String,
    pub hair_tip_mask: String,
}

#[derive(Serialize, Deserialize)]
pub struct Loadout {
    pub loadout_id: String,
    pub name: String,
    pub customization_options: LoadoutCustomizationOptions,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutCustomizationOptions {
    pub face_tattoo: AnimSet,
    pub pet: AnimSet,
    pub face_mask: AnimSet,
    pub makeup: AnimSet,
    pub torso: Torso,
    pub glider: AnimSet,
    pub hat: AnimSet,
    pub legs: Legs,
    pub face_complexion: AnimSet,
    pub body_complexion: AnimSet,
    pub body_tattoo: AnimSet,
}

#[derive(Serialize, Deserialize)]
pub struct Legs {
    pub id: String,
    pub variant: LegsVariant,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LegsVariant {
    pub legs_base_color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Torso {
    pub id: String,
    pub variant: TorsoVariant,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TorsoVariant {
    pub torso_base_color: String,
}