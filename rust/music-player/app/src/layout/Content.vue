<script setup lang="ts">


</script>

<template>
  <div class="content">
    <router-view v-slot="{ Component }">
      <transition name="fade">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </transition>
    </router-view>
  </div>
</template>

<style lang="css" scoped>
.content {
  padding: 50px 30px 30px 30px;
  height: 100%;
}

.fade-enter-active {
  transition: all 0.3s ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(10px);
}
</style>