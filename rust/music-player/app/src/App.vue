<script setup lang="ts">
import { watch } from 'vue';
import DefaultLayout from './layout/DefaultLayout.vue'
import { usePlayerStateStore } from './store/PlayerStateStore';
import { usePlayBottomStore } from './store/PlayBottomStore';
import { listen } from '@tauri-apps/api/event';
import PlayerEvents from './common/PlayerEvents';
import { message } from '@tauri-apps/plugin-dialog';

const bottomStore = usePlayBottomStore();
const playerStore = usePlayerStateStore();

bottomStore.hide();
watch(() => playerStore.playList, val => {
  val.length > 0 ? bottomStore.show() : bottomStore.hide();
});
listen(PlayerEvents.FileNoExists, async (event) => {
  const index = event.payload as number;
  if (playerStore.playList && index < playerStore.playList.length) {
    await message(`${playerStore.playList[index].title} 文件不存在`);
    playerStore.playList.splice(index, 1);
    playerStore.setPlayList(playerStore.playList);
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