<script setup lang="ts">
import SideMenu from './SideMenu.vue'
import Header from './Header.vue'
import Content from './Content.vue'
import PlayButtom from '../components/PlayBottom.vue';
import { ref, watch } from 'vue';
import { usePlayBottomStore } from '../store/PlayBottomStore';

const store = usePlayBottomStore();

const paddingBottom = ref("0px");
watch(() => store.playButtonShow, (value) => {
  if (value) {
    paddingBottom.value = "75px";
  } else {
    paddingBottom.value = "0px";
  }
}, { immediate: true });
</script>

<template>
  <div class="default-layout">
    <div class="wrapper">
      <SideMenu class="side-menu" />
      <div class="content-wrapper">
        <Header />
        <Content />
      </div>
    </div>
    <PlayButtom class="play-buttom" />
  </div>
</template>

<style lang="css" scoped>
.default-layout {
  --padding-bottom: v-bind(paddingBottom);

  position: relative;
  height: 100vh;

  .wrapper {
    width: 100%;
    display: flex;
    justify-content: space-between;

    .side-menu,
    .content-wrapper {
      overflow-y: auto;
      height: calc(100vh - var(--padding-bottom));
      transition: height var(--bottom-bar-duration) ease-in-out;
    }

    .content-wrapper {
      flex: 1;
      background-color: #F7F9FC;
    }
  }

  .play-buttom {
    width: 100%;
    height: 75px;
    background-color: #FAFAFA;
    position: absolute;
    bottom: 0;
  }
}
</style>