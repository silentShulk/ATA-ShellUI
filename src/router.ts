import { createRouter, createWebHistory } from 'vue-router'
import Home from './Pages/Home.vue'
import Install from './Pages/Install.vue'
import List from './Pages/Manage.vue'
import Uninstall from './Pages/Uninstall.vue'
import Settings from './Pages/Settings.vue'



const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Home },
    { path: '/install', component: Install },
    { path: '/manage', component: List },
    { path: '/uninstall', component: Uninstall },
		{ path: '/settings', component: Settings }, 
  ]
})

export default router