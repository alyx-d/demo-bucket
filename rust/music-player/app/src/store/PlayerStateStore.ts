import { defineStore } from "pinia";
import { ref } from "vue";
import { FileInfo, OwnFileInfo } from "../pages/LocalMusic.vue";
import { durationToSecs, last, storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";
import { invoke } from "@tauri-apps/api/core";
import Commands from "../common/Commands";

const getDefaultDirs = async (): Promise<string[]> => {
    const default_dirs = await invoke<string>(Commands.get_default_dirs);
    const dirs = default_dirs.split(",");
    storeSet(StorageKey.scan_dirs, dirs);
    return dirs;
};

export const doScanDirs = async (dirs: string[]): Promise<number> => {
    const result = await invoke<number>(Commands.player_scan_dirs, { dirs });
    return result;
};

export const readPlayList = async (): Promise<OwnFileInfo[]> => {
    const list = await invoke<FileInfo[]>(Commands.player_list);
    list.forEach((item, index) => {
        /// only save file name without extension
        item.path = last(item.path.split("\\")).replace(".mp3", "");
        (item as OwnFileInfo).originIndex = index;
    });
    storeSet(StorageKey.play_list, list);
    return list as OwnFileInfo[];
};

export const usePlayerStateStore = defineStore("player-state", () => {
    const isPlaying = ref(false);
    const playingIndex = ref<number>(
        storeGet<number>(StorageKey.playing_index) ?? 0,
    );
    const scanDirs = ref<string[] | null>(
        storeGet<string[]>(StorageKey.scan_dirs),
    );
    const playList = ref<OwnFileInfo[] | null>(
        storeGet<OwnFileInfo[]>(StorageKey.play_list),
    );

    const playingPos = ref(storeGet<number>(StorageKey.playing_pos) ?? 0);
    const totalDuration = ref(storeGet<number>(StorageKey.total_duration) ?? 0);
    const secTimer = ref<number | null>(null);

    if (!scanDirs.value) {
        getDefaultDirs().then((dirs) => {
            scanDirs.value = dirs;
            doScanDirs(dirs).then((_result) => {
                if (!playList.value) {
                    readPlayList().then((list) => {
                        playList.value = list as OwnFileInfo[];
                    });
                }
            });
        });
    }

    const setPlaying = (
        flag: boolean,
        index: number,
        isResume: boolean = false,
    ) => {
        isPlaying.value = flag;
        if (index >= 0 && index < playList.value!.length) {
            playingIndex.value = index;
            storeSet(StorageKey.playing_index, playingIndex.value);
        }
        if (secTimer.value) {
            clearInterval(secTimer.value);
            secTimer.value = null;
        }
        if (flag) {
            if (!isResume) {
                playingPos.value = 0;
            } else {
                playingPos.value = storeGet<number>(StorageKey.playing_pos) ??
                    0;
            }
            totalDuration.value = durationToSecs(
                playList.value![index].totalDuration,
            );
            storeSet(StorageKey.total_duration, totalDuration.value);
            const timer = setInterval(() => {
                if (playingPos.value >= totalDuration.value) {
                    clearInterval(timer);
                    secTimer.value = null;
                    playingPos.value = 0;
                } else {
                    playingPos.value += 1;
                }
                storeSet(StorageKey.playing_pos, playingPos.value);
            }, 1000);
            secTimer.value = timer;
        }
    };

    const setPlayingIndex = (index: number) => {
        playingIndex.value = index;
        storeSet(StorageKey.playing_index, index);
    };

    const setPlayList = (list: OwnFileInfo[]) => {
        playList.value = list;
        storeSet(StorageKey.play_list, list);
    };

    const setScanDirs = (dirs: string[]) => {
        scanDirs.value = dirs;
        storeSet(StorageKey.scan_dirs, dirs);
    };

    const setPlayingPos = (pos: number) => {
        playingPos.value = pos;
        storeSet(StorageKey.playing_pos, pos);
    };

    return {
        isPlaying,
        setPlaying,
        playingIndex,
        setPlayingIndex,
        scanDirs,
        setScanDirs,
        playList,
        setPlayList,
        playingPos,
        totalDuration,
        setPlayingPos,
    };
});
