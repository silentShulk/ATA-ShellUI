import { type ModType } from "../bindings";

export const getLabelByTipo = (tipo: ModType) => {
  return tipo.split(/(?=[A-Z])/)[0]
}

export const getIconaByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "settings";
    case "Textures": return "texture";
    case "PlayerModels": return "person";
    case "WeaponModels": return "swords";
    case "WorldModels": return "public";
    case "CutsceneReplacements": return "video_library";
    case "ReshadePreset": return "light_mode"
  }
}

export const getColorByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "settings";
    case "Textures": return "texture";
    case "PlayerModels": return "person";
    case "WeaponModels": return "swords";
    case "WorldModels": return "public";
    case "CutsceneReplacements": return "video_library";
    case "ReshadePreset": return "light_mode"
  }
}