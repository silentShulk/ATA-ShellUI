import { ModType } from "../bindings";

export const getCortoByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "DLL";
    case "Textures": return "Textures";
    case "PlayerModels": return "Pl. models";
    case "WeaponModels": return "We. models";
    case "WorldModels": return "Wo. models";
    case "CutsceneReplacements": return "Cutscenes";
    case "ReshadePreset": return "Reshade"
  }
}