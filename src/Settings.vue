<script setup lang="ts">
import { useRouter } from 'vue-router';
import {
    useSettingsStore,
    Style, Palette, SortingOrder, ConflictResolution,
} from './stores/settings'



const settingsStore = useSettingsStore()

const router = useRouter();
</script>



<template>
<div class="ata-page">
	<header class="ata-header">
		<h1 class="ata-title"> Options </h1>
	</header>

	<button class="ata-btn-back" @click="router.push('/')"> < </button>

	<main class="ata-main ata-flex-column">
	    <div class="ata-grid options-grid" v-if="settingsStore.settings">
			<h1 id="ui" class="section"> UI </h1>
			<div id="style">
			    <h3 class="option-name"> Style </h3>
				<select class="ata-select ata-colors"
				:value="settingsStore.settings.style" @change="settingsStore.setStyle(($event.target as HTMLSelectElement).value)">
				    <option class="ata-option" v-for="(style) in Style" :value="style">{{ style }}</option>
				</select>
			</div>
			<div id="palette">
			    <h3 class="option-name"> Color Palette </h3>
				<select class="ata-select ata-colors"
				:value="settingsStore.settings.palette" @change="settingsStore.setPalette(($event.target as HTMLSelectElement).value)">
				    <option v-for="(palette) in Palette" :value="palette">{{ palette }}</option>
				</select>
			</div>
			<div id="sorting">
			    <h3 class="option-name"> Mod Sorting </h3>
				<select class="ata-select ata-colors"
				:value="settingsStore.settings.sortingOrder" @change="settingsStore.setSortingOrder(($event.target as HTMLSelectElement).value)">
				    <option v-for="(order) in SortingOrder" :value="order">{{ order }}</option>
				</select>
			</div>

			<h1 id="files" class="section"> FILES </h1>
			<div id="conflict">
			    <h3 class="option-name"> Coflict Resolution </h3>
				<select class="ata-select ata-colors"
				:value="settingsStore.settings.filesConflictResolution" @change="settingsStore.setFilesConflictResolution(($event.target as HTMLSelectElement).value)">
				    <option v-for="(resolution) in ConflictResolution" :value="resolution">{{ resolution }}</option>
				</select>
			</div>
			<div id="keep">
			    <h3 class="option-name"> Keep Extracted<br>Folders </h3>
				<input type="checkbox" class="ata-checkbox ata-colors"
				:checked="settingsStore.settings.keepExtractedFolders" @change="settingsStore.setKeepExtractedFolders(($event.target as HTMLInputElement).checked)">
			</div>
			<div id="location">
			    <h3 class="option-name"> Extracted Folders<br>Location </h3>
				<textarea 
                class="ata-input-text ata-colors" 
                placeholder="/path/to/desired/location/"
                :value="settingsStore.settings.extractedFoldersLocation" 
                @change="settingsStore.setExtractedFoldersLocation(($event.target as HTMLTextAreaElement).value)">
            </textarea>
			</div>
			
			<h1 id="special" class="section"> SPECIAL </h1>
			<div id="override">
			    <h3 class="option-name"> Game Path </h3>
				<textarea
    			class="ata-input-text ata-colors"
    			placeholder="/path/to/exe/folder/"
    			:value="settingsStore.settings.gamePath"
    			@change="settingsStore.setGamePath(($event.target as HTMLTextAreaElement).value)">
				</textarea>
			</div>
			<div id="copy">
			    <h3 class="option-name"> Download Copy Of<br>Mod List/Settings </h3>
				<div>
					<button class="ata-btn ata-small ata-colors"> Download Copy<br>Of Mod List </button>
					<button class="ata-btn ata-small ata-colors"> Download Copy<br>Of Settings </button>
				</div>
			</div>
			<div id="discord">
			    <h3 class="option-name"> Discord<br>Rich Presence </h3>
					<textarea
    			class="ata-input-text ata-colors"
    			placeholder="/path/to/exe/folder/"
    			:value="settingsStore.settings.discordRichPresence"
    			@change="settingsStore.setDiscordRichPresence(($event.target as HTMLTextAreaElement).value)">
				</textarea>
			</div>	

			<div id="wipe">
                <button class="ata-btn ata-colors-critical btn-wipe">
                    <h3> WIPE<br>ALL MODS<br>AND<br>CONFIG DATA </h3>
                </button>
			</div>
		</div>
	</main>
</div>
</template>



<style scoped lang="scss">
.options-grid {
    grid-template-columns: repeat(3, 1fr);
    grid-template-rows: repeat(4, auto);
    grid-template-areas: 
        "ui files special"
        "style conflict override"
        "palette keep copy"
        "sorting location discord"
        ". wipe ."
}
.options-grid > * {
    display: flex;
    flex-direction: column;
    
    align-items: center;
    justify-content: center;

    margin: 0;
    padding: 0;
}

#ui { grid-area: ui; }
#files { grid-area: files; }
#special { grid-area: special; }
#style { grid-area: style; }
#palette { grid-area: palette; }
#sorting { grid-area: sorting; }
#conflict { grid-area: conflict; }
#keep { grid-area: keep; }
#location { grid-area: location; }
#override { grid-area: override; }
#copy { grid-area: copy; }
#discord { grid-area: discord; }
#wipe { grid-area: wipe; }

.section {
    color: $ata-accent-secondary;

    margin-bottom: 10px;
}

.option-name {
    margin: 10px;

    font-size: 20px;
}

.btn-wipe {
    margin: 0 100px 0 100px;
}
</style>