import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Path } from 'typescript'



export enum Style {
    SilentShulk = 'SilentShulk',
    Beyluta = 'Beyluta',
}

export enum Palette {
    Automata = 'Automata',
    Replicant = 'Replicant',
}

export enum SortingOrder {
    ModType = "ModType",
    EnableStatus = "Enable Status",
    Alphabetical = "Alphabetical",
    InstallDate = "Install Date",
    Size = "Size",
}

export enum ConflictResolution {
    Ask = "Ask",
    Overwrite = "Overwrite",
    Skip = "Skip",
}

export interface Settings {
    /// Visual style / layout theme
    style: Style,
    /// Color palette
    palette: Palette,
    /// Order in which mods are shown in the list
    sortingOrder: SortingOrder,

    /// How to handle file conflicts during installation
    filesConflictResolution: ConflictResolution,
    /// Whether to keep the extracted temp folder after installation
    keepExtractedFolders: boolean,
    /// Location to save extracted folders
    extractedFoldersLocation: Path,

    /// Path to the game's installation folder 
    gamePath: Path,
    /// Discord Rich Presence application ID (empty string = disabled)
    discordRichPresence: string,
}


export const useSettingsStore = defineStore('settings', () => {
    const settings = ref<Settings>({} as Settings)

    async function load_settings() {
        settings.value = await invoke('load_settings_command')

        applyAll()
    }

    function applyAll() {
        document.documentElement.classList.add(settings.value.palette)
    }

    async function update_setting(setting: string, value: string) {
        settings.value = await invoke('update_setting_command', { setting, value })

        applyAll()
    }

    return { settings, load_settings, update_setting }
})