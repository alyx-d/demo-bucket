<script setup lang="ts">

</script>

<template>
    <router-view v-slot="{ Component }">
        <transition name="fade">
            <keep-alive>
                <component :is="Component" />
            </keep-alive>
        </transition>
    </router-view>
</template>

<style lang="css" scoped>
.fade-enter-active {
    transition: all 0.3s ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
    transform: translateX(10px);
}
</style>