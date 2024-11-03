import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router"

const routes: Readonly<RouteRecordRaw[]> = [
    {
        path: "/",
        component: () => import("../pages/Recommend.vue"),
    },
    {
        path: "/recommend",
        component: () => import("../pages/Recommend.vue"),
    },
    {
        path: "/podcast",
        component: () => import("../pages/Podcast.vue"),
    },
    {
        path: "/community",
        component: () => import("../pages/Community.vue"),
    },

]

export const router = createRouter({
    history: createWebHistory(),
    routes,
});