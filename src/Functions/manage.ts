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

export const getColoreByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "grey";
    case "Textures": return "blue";
    case "PlayerModels": return "light";
    case "WeaponModels": return "accent-secondary-dark";
    case "WorldModels": return "green";
    case "CutsceneReplacements": return "magenta";
    case "ReshadePreset": return "main"
  }
}

export const getSfondoByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "bg-grey";
    case "Textures": return "bg-blue";
    case "PlayerModels": return "bg-light";
    case "WeaponModels": return "bg-accent-secondary-dark";
    case "WorldModels": return "bg-green";
    case "CutsceneReplacements": return "bg-magenta";
    case "ReshadePreset": return "bg-main"
  }
}

export const getAccentoByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "var(--dark)";
    case "Textures": return "var(--light)";
    case "PlayerModels": return "var(--dark)";
    case "WeaponModels": return "var(--light)";
    case "WorldModels": return "var(--light)";
    case "CutsceneReplacements": return "var(--light)";
    case "ReshadePreset": return "var(--dark)";
  }
}
export const getAccentoOppostoByTipo = (tipo: ModType) => {
  switch (tipo) {
    case "DLL": return "var(--light)";
    case "Textures": return "var(--dark)";
    case "PlayerModels": return "var(--light)";
    case "WeaponModels": return "var(--dark)";
    case "WorldModels": return "var(--dark)";
    case "CutsceneReplacements": return "var(--dark)";
    case "ReshadePreset": return "var(--light)";
  }
}