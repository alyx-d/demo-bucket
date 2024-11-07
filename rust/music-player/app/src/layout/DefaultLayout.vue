<script setup lang="ts">
import SideMenu from './SideMenu.vue'
import Header from './Header.vue'
import Content from './Content.vue'
</script>

<template>
  <div class="default-layout">
    <SideMenu/>
    <div class="content">
      <Header/>
      <Content/>
    </div>
  </div>
</template>

<style lang="css" scoped>
.default-layout {
  width: 100%;
  height: 100vh;
  display: flex;
  justify-content: space-between;

  .content {
    flex-grow: 1;
  }
}
</style>