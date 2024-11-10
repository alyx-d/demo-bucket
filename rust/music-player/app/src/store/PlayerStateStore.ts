import { defineStore } from "pinia";
import { ref } from "vue";
import { durationToSecs, last, storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";
import { invoke } from "@tauri-apps/api/core";
import Commands from "../common/Commands";
import { FileInfo, OwnFileInfo } from "../types/Components.interface";
import { useUserSelectDirStore } from "./UserSelectDirStore";

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
        const fileName = last(item.path.split("\\")).replace(".mp3", "");
        if (item.title == "未知歌曲") {
            item.title = fileName;
        }
        (item as OwnFileInfo).originIndex = index;
    });
    return list as OwnFileInfo[];
};

export const usePlayerStateStore = defineStore("player-state", () => {
    const isPlaying = ref(false);
    const isPause = ref(false);
    const playingIndex = ref<number>(
        storeGet<number>(StorageKey.playing_index) ?? -1,
    );

    const scanDirs = ref<string[]>(
        storeGet<string[]>(StorageKey.scan_dirs) ?? [],
    );
    const playList = ref<OwnFileInfo[]>([]);

    const playingPos = ref(0);
    const totalDuration = ref(0);
    const secTimer = ref<number | null>(null);

    const scanAndList = (dirs: string[]) => {
        const userSelectDirStore = useUserSelectDirStore();
        userSelectDirStore.setUserSelectDirs(dirs.map((dir, _index) => {
            return {
                path: dir,
                checked: true,
            };
        }));
        doScanDirs(dirs).then((_result) => {
            if (playList.value.length == 0) {
                readPlayList().then((list) => {
                    playList.value = list as OwnFileInfo[];
                    console.log("read play list", playList.value);
                });
            }
        });
    };

    if (scanDirs.value.length == 0) {
        getDefaultDirs().then((dirs) => {
            scanDirs.value = dirs;
            console.log("scanAndList", dirs);
            scanAndList(dirs);
        });
    } else {
        console.log("has dirs");
        scanAndList(scanDirs.value);
    }

    const setPlaying = (
        flag: boolean,
        index: number,
        isResume: boolean = false,
    ) => {
        isPlaying.value = flag;
        if (index >= 0 && index < playList.value!.length) {
            playingIndex.value = index;
        }
        if (secTimer.value) {
            clearInterval(secTimer.value);
            secTimer.value = null;
        }
        if (flag) {
            if (!isResume) {
                playingPos.value = 0;
            }
            totalDuration.value = durationToSecs(
                playList.value![index].totalDuration,
            );
            const timer = setInterval(() => {
                if (playingPos.value >= totalDuration.value) {
                    clearInterval(timer);
                    secTimer.value = null;
                    playingPos.value = 0;
                } else {
                    playingPos.value += 1;
                }
            }, 1000);
            secTimer.value = timer;
        }
    };

    const isPlayCompleted = () => {
        return playingPos.value >= totalDuration.value;
    };

    const setPause = (flag: boolean) => {
        isPause.value = flag;
    };

    const setPlayingIndex = (index: number) => {
        playingIndex.value = index;
        storeSet(StorageKey.playing_index, index);
    };

    const setPlayList = (list: OwnFileInfo[]) => {
        playList.value = list;
    };

    const setScanDirs = (dirs: string[]) => {
        scanDirs.value = dirs;
        storeSet(StorageKey.scan_dirs, dirs);
    };

    const setPlayingPos = (pos: number) => {
        playingPos.value = pos;
    };

    const setTotalDuration = (duration: number) => {
        totalDuration.value = duration;
    };

    return {
        isPlaying,
        setPlaying,
        isPause,
        setPause,
        playingIndex,
        setPlayingIndex,
        scanDirs,
        setScanDirs,
        playList,
        setPlayList,
        playingPos,
        totalDuration,
        setPlayingPos,
        setTotalDuration,
        isPlayCompleted,
    };
});
