<script setup lang="ts">
import { onActivated, ref, watchEffect } from "vue";
import Dialog, { ExposeMethods } from "../components/Dialog.vue";
import SelectLocalDir from "../components/SelectLocalDir.vue";
import { invoke } from "@tauri-apps/api/core";
import Commands from "../common/Commands.ts";
import { listen } from "@tauri-apps/api/event";
import PlayerEvents from "../common/PlayerEvents.ts";
import { doScanDirs, readPlayList, usePlayerStateStore } from "../store/PlayerStateStore.ts";
import { OwnFileInfo, PlayerCtl } from "../types/Components.interface.ts";

const scanTotal = ref(0);
const scanShow = ref(false);
const selectedItemIdx = ref(-1);

const playerStore = usePlayerStateStore();

const playList = ref<OwnFileInfo[]>(playerStore.playList);

const playerCtl = ref<PlayerCtl>({
  currentIndex: -1,
  isPlaying: playerStore.isPlaying,
  isPause: playerStore.isPause,
});
watchEffect(() => {
  if (playList.value.length != playerStore.playList.length) {
    playList.value = playerStore.playList;
  }
  playerCtl.value.isPlaying = playerStore.isPlaying;
  playerCtl.value.isPause = playerStore.isPause;
});

const dialog = ref<ExposeMethods | null>(null);
const openDialog = () => {
  if (dialog.value) {
    dialog.value.open();
  }
};

const unDisplayScan = (num: number) => {
  scanTotal.value = num;
  scanShow.value = true;
  setTimeout(() => {
    scanShow.value = false;
  }, 1500);
};

onActivated(async () => {
  console.log("onActivated");
  if (playerStore.scanDirs) {
    const num = await doScanDirs(playerStore.scanDirs);
    unDisplayScan(num);
    if (num) {
      const list = await readPlayList();
      playerStore.setPlayList(list);
    }
  }
});

const onPlayClick = async (index: number, originIndex: number) => {
  if (playerCtl.value.isPause && index == playerCtl.value.currentIndex) {
    await invoke(Commands.player_resume);
  } else {
    await invoke(Commands.player_play_index, { index: originIndex });
  }
};

const onPauseClick = async () => {
  await invoke(Commands.player_pause);
};

const onDbClick = async (index: number, originIndex: number) => {
  if (playerCtl.value.currentIndex != index) {
    await invoke(Commands.player_play_index, { index: originIndex });
  } else {
    if (playerCtl.value.isPause) {
      await invoke(Commands.player_resume);
    } else {
      await invoke(Commands.player_pause);
    }
  }
};

const isPlaying = (index: number) => {
  return playerCtl.value.isPlaying && playerCtl.value.currentIndex == index;
};

const isPlayingClass = (index: number): string => {
  return (playerCtl.value.isPlaying || playerCtl.value.isPause) && playerCtl.value.currentIndex == index ? "playing" : "";
};

const isPauseClass = (index: number): string => {
  return playerCtl.value.isPause && playerCtl.value.currentIndex == index ? "pause" : "";
};

listen(PlayerEvents.Play, (event) => {
  const index = event.payload as number;
  for (let idx in playList.value) {
    if (playList.value[idx].originIndex == index) {
      playerCtl.value.currentIndex = Number(idx);
      break;
    }
  }
});

const onItemClick = (index: number) => {
  selectedItemIdx.value = index;
};

const selectedClass = (index: number): string => {
  return selectedItemIdx.value == index ? "selected" : "";
};
</script>

<template>
  <div class="local-music">
    <div class="wrapper">
      <div class="title">
        <h2>本地音乐</h2>
        <span class="total-music">共 {{ playList.length }} 首</span>
        <span v-if="scanShow" class="scan">成功匹配到 {{ scanTotal }} 首歌曲</span>
      </div>
      <div class="select-dirs">
        <span class="text" @click="openDialog">选择目录 ></span>
        <Dialog ref="dialog">
          <SelectLocalDir />
        </Dialog>
      </div>
    </div>
    <div class="play-list">
      <div class="item head">
        <div class="seq">#</div>
        <div class="title">标题</div>
        <div class="album">专辑</div>
        <div class="total-duration">时长</div>
        <div class="size">大小</div>
      </div>
      <div :class="`item ${selectedClass(index)}`" v-for="(item, index) in playList" :key="index"
        @click="onItemClick(index)" @dblclick="onDbClick(index, item.originIndex)">
        <div :class="`seq ${isPauseClass(index)}`">
          <span v-show="!isPlaying(index)" class="text">{{ (index + 1).toString().padStart(2, "0") }}</span>
          <img v-show="isPlaying(index)" class="text" src="/icons/playing.svg" alt="playing" />
          <img v-show="!isPlaying(index)" class="play" src="/icons/play_fill.svg" alt="play"
            @click="onPlayClick(index, item.originIndex)" />
          <img v-show="isPlaying(index)" class="play" src="/icons/pause.svg" alt="play" @click="onPauseClick" />
        </div>
        <div :class="`title ${isPlayingClass(index)}`">{{ item.title }}</div>
        <div class="album">{{ item.album }}</div>
        <div class="total-duration">{{ item.totalDuration }}</div>
        <div class="size">{{ item.mb }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="css">
.local-music {
  width: 100%;

  .wrapper {
    width: 100%;
    display: flex;
    justify-content: space-between;

    .title {
      display: flex;
      align-items: baseline;

      .total-music,
      .scan {
        margin-left: 10px;
        height: max-content;
        font-size: 12px;
        color: #ccc;
      }
    }

    .select-dirs {

      .text {
        font-size: 14px;
        font-weight: bold;
        color: #6176af;
        cursor: pointer;
      }
    }

  }

  .play-list {
    padding-top: 20px;

    .item {
      display: flex;
      align-items: center;
      padding: 15px 20px;
      font-size: 13px;

      &:not(.head):hover {
        background-color: #fff;
        border-radius: 10px;

        .play {
          opacity: 1;
        }

        .text {
          opacity: 0;
        }
      }

      &.selected {
        background-color: #fff;
        border-radius: 10px;

        .text {
          opacity: 1;
        }
      }

      .seq {
        width: 40px;
        text-align: center;
        font-size: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;

        .play,
        .text {
          position: absolute;
          top: 50%;
          transform: translateY(-50%);
          width: 15px;
          height: 15px;
          transition: opacity 10ms ease-in-out;
        }


        .play {
          opacity: 0;
          cursor: pointer;
        }
      }

      .total-duration,
      .size {
        width: 75px;
      }

      .title {
        flex: 1.6;
      }

      .album {
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }
  }

  .playing {
    color: red;
  }

  .pause {
    .text {
      opacity: 0 !important;
    }

    .play {
      opacity: 1 !important;
    }
  }
}

@keyframes reverse {

  0% {
    transform: scaleX(1);
  }

  49.9% {
    transform: scaleX(1);
  }

  50% {
    transform: scaleX(-1);
  }

  100% {
    transform: scaleX(-1);
  }
}
</style>