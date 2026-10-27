use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Mod {
    pub name: String,
    pub files: Vec<PathBuf>,
    pub enabled: bool,
    pub mod_type: ModType,
    pub install_date: DateTime<Utc>,
    pub uid: String,
}
impl Mod {
    pub fn new(
        name: String,
        files: Vec<PathBuf>,
        enabled: bool,
        mod_type: ModType,
        install_date: DateTime<Utc>,
    ) -> Self {
        Self {
            uid: Self::get_uid(&name, &mod_type, &install_date),
            name,
            files,
            enabled,
            mod_type,
            install_date,
        }
    }

    fn get_uid(mod_name: &str, mod_type: &ModType, install_date: &DateTime<Utc>) -> String {
        let name: String = mod_name.chars().take(4).collect();
        let m_type = mod_type.get_id();
        let date = install_date.format("%d/%m/%Y|%H:%M").to_string();

        format!("{}{}{}", name, m_type, date)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ModType {
    DLL,
    Textures,
    PlayerModels,
    WeaponModels,
    WorldModels,
    CutsceneReplacements,
    ReshadePreset,
}
impl ModType {
    fn get_id(&self) -> &str {
        match self {
            ModType::DLL => "Dll",
            ModType::Textures => "Te",
            ModType::PlayerModels => "PlMo",
            ModType::WeaponModels => "WeMo",
            ModType::WorldModels => "WoMo",
            ModType::CutsceneReplacements => "CuRe",
            ModType::ReshadePreset => "RePr",
        }
    }
}