import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { routes } from "./router/routes.js";
import * as VueRouter from 'vue-router';
import { createPinia } from 'pinia';



const router = VueRouter.createRouter({
    history: VueRouter.createWebHistory(),
    routes,
});

const pinia = createPinia();

createApp(App)
    .use(router)
    .use(pinia)
    .mount('#app');