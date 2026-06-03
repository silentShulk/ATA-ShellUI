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

    async function load() {
        settings.value = await invoke('get_settings')

        applyAll()
    }

    function applyAll() {
        document.documentElement.classList.add(settings.value.palette)
    }

    async function setStyle(selected: string) {
        await invoke('set_style', { style: selected })
    
        const style = selected as Style
        // Frontend style change
        settings.value.style = style
    }
    async function setPalette(selected: string) {
        const oldTheme = settings.value.palette

        await invoke('set_palette', { palette: selected })
        
        const palette = selected as Palette
        settings.value.palette = palette
    
        document.documentElement.classList.remove(oldTheme)
        document.documentElement.classList.add(selected)
    }
    async function setSortingOrder(selected: string) {
        await invoke('set_sorting_order', { sortingOrder: selected })
    
        const sortingOrder = selected as SortingOrder
        settings.value.sortingOrder = sortingOrder
    }
    async function setFilesConflictResolution(selected: string) {
        await invoke('set_files_conflict_resolution', { filesConflictResolution: selected })
    
        const filesConflictResolution = selected as ConflictResolution
        settings.value.filesConflictResolution = filesConflictResolution
    }
    async function setKeepExtractedFolders(selected: boolean) {
        await invoke('set_keep_extracted_folders', { keepExtractedFolders: selected })
    
        settings.value.keepExtractedFolders = selected
    }
    async function setExtractedFoldersLocation(selected: string) {
        await invoke('set_extracted_folders_location', { extractedFoldersLocation: selected })
    
        const extractedFoldersLocation = selected as Path
        settings.value.extractedFoldersLocation = extractedFoldersLocation
    }
    async function setGamePath(selected: string) {
        await invoke('set_game_path', { gamePath: selected })
    
        const gamePath = selected as Path
        settings.value.gamePath = gamePath
    }
    async function setDiscordRichPresence(selected: string) {
        await invoke('set_discord_rich_presence', { discordRichPresence: selected })
    
        const discordRichPresence = selected
        settings.value.discordRichPresence = discordRichPresence
    }

    return { settings, load, setStyle, setPalette, setSortingOrder, setFilesConflictResolution, setKeepExtractedFolders, setExtractedFoldersLocation, setGamePath, setDiscordRichPresence }
})



