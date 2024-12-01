import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";

const routes: Readonly<RouteRecordRaw[]> = [
    {
        path: "/",
        name: "index",
        redirect: "/page1",
    },
    {
        path: "/page1",
        name: "page1",
        component: () => import("@/views/page1.vue"),
    },
    {
        path: "/page2",
        name: "page2",
        component: () => import("@/views/page2.vue"),
    }
];

export const router = createRouter({
    history: createWebHashHistory(),
    routes,
});