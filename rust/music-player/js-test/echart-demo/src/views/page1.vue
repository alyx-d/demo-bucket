<script setup lang="ts">
import { useAppStore } from '@/store/appStore';
import { useDark, useTitle } from '@vueuse/core';
import { ref } from 'vue';


const appStore = useAppStore();
const isDark = useDark();
const title = useTitle();
const loading = ref(false);

const onClickLoading = () => {
    loading.value = true;
    setTimeout(() => {
        loading.value = false;
    }, 2000);
};

</script>

<template>
    <div class="page1" v-loading="loading" element-loading-text="loading...">
        <h1>page1</h1>
        <h2>isDark is {{ isDark }}</h2>
        <h2>count is {{ appStore.count }}</h2>
        <el-button @click="appStore.increment">increment</el-button>
        <el-button @click="title = `page1 ${appStore.count}`">change title</el-button>
        <br />
        <router-link to="/page2">page2</router-link>
        <br />
        <el-button @click="onClickLoading">loading</el-button>
    </div>
</template>

<style scoped lang="css">
.page1 {
    width: max-content;
    margin: auto;
}
</style>