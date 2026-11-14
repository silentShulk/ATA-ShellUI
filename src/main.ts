import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import "./Styles/Classes/alignments.scss";
import "./Styles/Classes/borders.scss"
import "./Styles/Classes/color-combos.scss"
import "./Styles/Classes/shadows.scss";
import "./Styles/Classes/typography.scss";
import "./Styles/Classes/Spacings/gap.scss";
import "./Styles/Classes/Spacings/margin.scss";
import "./Styles/Classes/Spacings/padding.scss";
import "./Styles/Components/button.scss";
import "./Styles/Components/input-text.scss";
import '@fontsource-variable/material-symbols-rounded'
import '@fontsource-variable/jetbrains-mono'

createApp(App).use(router).mount("#app");