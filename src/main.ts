import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./Styles/typography.scss";
import "./Styles/Classes/button.scss";
import "./Styles/Classes/color-combos.scss";

createApp(App).use(router).mount("#app");