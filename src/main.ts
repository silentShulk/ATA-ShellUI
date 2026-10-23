import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./Styles/Classes/typography.scss";
import "./Styles/Classes/button.scss";
import "./Styles/Classes/color-combos.scss";
import '@fontsource/material-symbols-rounded'
import '@fontsource/jetbrains-mono/400.css'
import '@fontsource/jetbrains-mono/700.css'

createApp(App).use(router).mount("#app");