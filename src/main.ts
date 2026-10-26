import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./Styles/Classes/typography.scss";
import "./Styles/Classes/button.scss";
import "./Styles/Classes/input-text.scss"
import "./Styles/Classes/color-combos.scss";
import "./Styles/Classes/Spacings/margin.scss"
import "./Styles/Classes/Spacings/padding.scss"
import "./Styles/Classes/Spacings/gap.scss"
import '@fontsource/material-symbols-rounded'
import '@fontsource/jetbrains-mono/400.css'
import '@fontsource/jetbrains-mono/700.css'

createApp(App).use(router).mount("#app");