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
    <div class="wrapper">
      <div class="nav">
        <router-link draggable="false" :class="isActive('/recommend')" to="/recommend">云音乐精选</router-link>
        <router-link draggable="false" :class="isActive('/podcast')" to="/podcast">博客</router-link>
        <router-link draggable="false" :class="isActive('/community')" to="/community">社区</router-link>
      </div>
    </div>
    <div class="device-line">
      <hr />
    </div>
    <div class="wrapper">
      <div class="tip">
        <span class="text">我的</span>
      </div>
      <div class="nav">
        <router-link draggable="false" :class="isActive('/local-music')" to="/local-music">本地音乐</router-link>
      </div>
    </div>
  </div>
</template>

<style lang="css" scoped>
.side-menu {
  width: 200px;
  min-width: 200px;
  height: 100vh;
  min-height: 100vh;
  background-color: #f0f3f6;

  .wrapper {
    width: 100%;
    padding: 0 30px;

    .tip {
      padding: 0 10px 20px 10px;

      .text {
        font-size: 12px;
        color: #999;
      }
    }
  }

  .nav {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;


    a {
      text-align: start;
      width: 100%;
      padding: 7px 10px;
      border-radius: 8px;
      font-size: 15px;

      &:not(:last-child) {
        margin-bottom: 5px;
      }

      &:hover {
        background-color: #eee;
      }

      &.active {
        color: white;
        background: linear-gradient(to right, #f6142e, #f85354);
      }
    }
  }

  .device-line {
    padding: 20px 30px;

    hr {
      height: 1px;
      background-color: #eee;
      border: none;
    }
  }
}
</style>