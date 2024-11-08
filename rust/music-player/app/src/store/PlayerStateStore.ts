import { defineStore } from "pinia";
import { ref } from "vue";
import { FileInfo, OwnFileInfo } from "../pages/LocalMusic.vue";
import { last, storeGet, storeSet } from "../common/Utils";
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

    const setPlaying = (flag: boolean) => {
        isPlaying.value = flag;
    };

    const setPlayingIndex = (index: number) => {
        playingIndex.value = index;
    };

    const setPlayList = (list: OwnFileInfo[]) => {
        playList.value = list;
        storeSet(StorageKey.play_list, list);
    };

    const setScanDirs = (dirs: string[]) => {
        scanDirs.value = dirs;
        storeSet(StorageKey.scan_dirs, dirs);
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
    };
});
