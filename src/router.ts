import { createRouter, createWebHistory } from 'vue-router'
import Home from './Home.vue'
import Install from './Install.vue'
import List from './List.vue'
import Uninstall from './Uninstall.vue'
import Browse from './Browse.vue'
import Settings from './Settings.vue'



const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/install', component: Install },
    { path: '/list', component: List },
    { path: '/uninstall', component: Uninstall },
		{ path: '/browse', component: Browse }, 
    { path: '/settings', component: Settings }
  ]
})

export default router