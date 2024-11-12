<script setup lang="ts">
import DefaultLayout from './layout/DefaultLayout.vue'
import { usePlayerStateStore } from './store/PlayerStateStore';
import { listen } from '@tauri-apps/api/event';
import PlayerEvents from './common/PlayerEvents';
import { message } from '@tauri-apps/plugin-dialog';

const playerStore = usePlayerStateStore();

listen(PlayerEvents.FileNoExists, async (event) => {
  const index = event.payload as number;
  if (playerStore.playList && index < playerStore.playList.length) {
    await message(`${playerStore.playList[index].title} 文件不存在`);
    playerStore.playList.splice(index, 1);
    playerStore.setPlayList(playerStore.playList);
  }
});

const getTheme = (): string => {
  return document.documentElement.getAttribute("auto") || "";
}

const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
mediaQuery.addEventListener("change", (e) => {
  console.log(e);
  if (getTheme() === "auto") {
    if (e.matches) {
      document.documentElement.setAttribute("data-theme", "dark");
    } else {
      document.documentElement.setAttribute("data-theme", "light");
    }
  }
});
</script>

<template>
  <main>
    <DefaultLayout />
  </main>
</template>

<style lang="css" scoped>
main {
  min-width: 800px;
  min-height: 600px;

}
</style>