import { createApp } from 'vue'
import './style.css'
import "element-plus/dist/index.css";
import ElementPlus from "element-plus";
import App from './App.vue'
import { router } from './router/router';
import { createPinia } from 'pinia';

const pinia = createPinia();

createApp(App)
    .use(router)
    .use(ElementPlus)
    .use(pinia)
    .mount('#app')
