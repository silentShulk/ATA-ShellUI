import { Path } from "typescript";

export interface Mod {
    name: string;
    files: Path[];
    enabled: boolean;
    modType: ModType;
    installDate: string;
    uid: string;
}

export enum ModType {
    DLL = "DLL",
    Textures = "Textures",
    PlayerModels = "PlayerModels",
    WeaponModels = "WeaponModels",
    WorldModels = "WorldModels",
    CutsceneReplacements = "CutsceneReplacements",
    ReshadePreset = "ReshadePreset",
}