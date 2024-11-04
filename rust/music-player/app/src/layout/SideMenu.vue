<script setup lang="ts">
import { useRoute } from 'vue-router';
import AppName from './AppName.vue';
import { ref, watch } from 'vue';

const route = useRoute();
const active = ref("/recommend");
watch(() => route.path, value => {
    active.value = value;
});
const isActive = (path: string): string => {
    return active.value === path ? "active" : "";
};

</script>

<template>
    <div class="side-menu">
        <AppName />
        <div class="wrpper">
            <div class="first-nav">
                <router-link :class="isActive('/recommend')" to="/recommend">云音乐精选</router-link>
                <router-link :class="isActive('/podcast')" to="/podcast">博客</router-link>
                <router-link :class="isActive('/community')" to="/community">社区</router-link>
            </div>
        </div>
        <div class="devide-line">
            <hr />
        </div>
    </div>
</template>

<style lang="css" scoped>
.side-menu {
    width: 250px;
    height: 100%;

    .wrpper {
        width: 100%;
        padding: 0 30px;
    }

    .first-nav {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;

        a {
            text-align: start;
            width: 100%;
            padding: 7px 10px;
            border-radius: 8px;
            font-size: 14px;

            &:not(:last-child) {
                margin-bottom: 5px;
            }

            &:hover {
                background-color: #eee;
            }

            &.active {
                color: white;
                background-color: #ff0011;
            }
        }
    }

    .devide-line {
        padding: 20px 30px;

        hr {
            height: 1px;
            background-color: #eee;
            border: none;
        }
    }
}
</style>