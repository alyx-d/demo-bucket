<script setup lang="ts">
import AppName from './AppName.vue'
</script>

<template>
    <div class="side-menu">
        <AppName />
        <div class="first-nav">
            <router-link to="/">Recommend</router-link>
            <router-link to="/podcast">Podcast</router-link>
            <router-link to="/community">Community</router-link>
        </div>
    </div>
</template>

<style lang="css" scoped>
.side-menu {
    width: 200px;
    height: 100%;

    .first-nav {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;

        a {
            width: 100%;
            margin-left: 30px;
        }
    }
}
</style>