<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
import Dialog, { ExposeMethods } from "../components/Dialog.vue";
import SelectLocalDir from "../components/SelectLocalDir.vue";
import { invoke } from "@tauri-apps/api/core";
import StorageKey from "../common/StorageKey.ts";
import Commands from "../common/Commands.ts";
import { last } from "../common/Utils.ts";
import { listen } from "@tauri-apps/api/event";
import PlayerEvents from "../common/PlayerEvents.ts";

export interface FileInfo {
  path: string;
  totalDuration: string;
  size: string;
  mb: string;
  title: string;
  artist: string;
  album: string;
}

interface PlayerCtl {
  currentIndex: number;
  isPlaying: boolean;
}

const totalMusic = ref(0);
const scanTotal = ref(0);
const scanShow = ref(false);

const playList = ref<FileInfo[]>([]);

const playerCtl = ref<PlayerCtl>({ currentIndex: 0, isPlaying: false });

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

const scanDefaultDirs = async () => {
  const scan_dirs_str = localStorage.getItem(StorageKey.scan_dirs);
  let dirs: string[] = [];
  if (!scan_dirs_str) {
    const default_dirs = await invoke(Commands.get_default_dirs) as string;
    dirs = default_dirs.split(",");
    localStorage.setItem(StorageKey.scan_dirs, JSON.stringify(dirs));
  } else {
    dirs = JSON.parse(scan_dirs_str) as string[];
  }
  const result = await invoke(Commands.player_scan_dirs, { dirs }) as number;
  await invoke(Commands.player_pause);
  unDisplayScan(result);
};

const readPlayList = async () => {
  const localList = localStorage.getItem(StorageKey.play_list);
  let list: FileInfo[] = [];
  if (!localList) {
    list = await invoke(Commands.player_list) as FileInfo[];
    console.log("list:", list);
    localStorage.setItem(StorageKey.play_list, JSON.stringify(list));
  } else {
    list = JSON.parse(localList) as FileInfo[];
  }
  totalMusic.value = list.length;
  if (list.length > 0) {
    playList.value = list.map(it => {
      it.path = last(it.path.split("\\")).replace(".mp3", "");
      return it;
    });
  }
};
onBeforeMount(async () => {
  localStorage.clear();
  await scanDefaultDirs();
  await readPlayList();
});

const play = async (index: number) => {
  playerCtl.value.isPlaying = true;
  playerCtl.value.currentIndex = index - 1;
  await invoke(Commands.player_set_volume, { volume: 1.0 });
  await invoke(Commands.player_play_index, { index });
};

const pause = async () => {
  playerCtl.value.isPlaying = false;
  await invoke(Commands.player_pause);
};

const isPlaying = (index: number) => {
  return playerCtl.value.isPlaying && playerCtl.value.currentIndex == index;
};

const isPlayingClass = (index: number) => {
  return isPlaying(index) ? "playing" : "";
};

listen(PlayerEvents.Play, (event) => {
  console.log("PlayerEvents.Play", event);
  playerCtl.value.currentIndex = event.payload as number;
});
</script>

<template>
  <div class="local-music">
    <div class="wrapper">
      <div class="title">
        <h2>本地音乐</h2>
        <span class="total-music">共 {{ totalMusic }} 首</span>
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
      <div class="item" v-for="(item, index) in playList" :key="index">
        <div class="seq">
          <span v-show="!isPlaying(index)" class="text">{{ (index + 1).toString().padStart(2, "0") }}</span>
          <img v-show="isPlaying(index)" class="text" src="/icons/playing.svg" alt="playing" />
          <img v-show="!isPlaying(index)" class="play" src="/icons/play_fill.svg" alt="play" @click="play(index + 1)" />
          <img v-show="isPlaying(index)" class="play" src="/icons/pause.svg" alt="play" @click="pause" />
        </div>
        <div :class="`title ${isPlayingClass(index)}`">{{ item.path }}</div>
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
  height: 100%;

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
      align-items: baseline;
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

      .seq {
        width: 40px;
        text-align: center;
        font-size: 12px;
        display: flex;
        align-items: center;
        position: relative;

        .play,
        .text {
          position: absolute;
          top: -12px;
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
}
</style>