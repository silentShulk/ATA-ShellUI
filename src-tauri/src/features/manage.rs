use crate::data::mods::Mod;

pub fn list_mods_inner(mods: &[Mod]) -> Vec<Mod> {
    let sorted_mods: Vec<Mod> = mods.into();

    sorted_mods
}
