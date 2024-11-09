<script setup lang="ts">
import { watch } from 'vue';
import DefaultLayout from './layout/DefaultLayout.vue'
import { usePlayerStateStore } from './store/PlayerStateStore';
import { usePlayBottomStore } from './store/PlayBottomStore';
import { listen } from '@tauri-apps/api/event';
import PlayerEvents from './common/PlayerEvents';

const bottomStore = usePlayBottomStore();
const playerStore = usePlayerStateStore();
watch(() => playerStore.playingIndex, val => {
  val > -1 ? bottomStore.show() : bottomStore.hide();
});
listen(PlayerEvents.FileNoExists, event => {
  const index = event.payload as number;
  if (playerStore.playList && index < playerStore.playList.length) {
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