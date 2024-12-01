<script setup lang="ts">
import { useAppStore } from '@/store/appStore';
import { useDark, useMouse, useTitle } from '@vueuse/core';


const appStore = useAppStore();
const isDark = useDark();
const { x, y } = useMouse();
const title = useTitle();

</script>

<template>
    <div class="page1">
        <h1>page1</h1>
        <h2>x is {{ x }}</h2>
        <h2>y is {{ y }}</h2>
        <h2>isDark is {{ isDark }}</h2>
        <h2>count is {{ appStore.count }}</h2>
        <button @click="appStore.increment">increment</button>
        <button @click="title = `page1 ${appStore.count}`">change title</button>
        <router-link to="/page2">page2</router-link>
    </div>
</template>