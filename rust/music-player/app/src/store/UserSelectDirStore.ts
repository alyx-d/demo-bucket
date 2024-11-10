import { defineStore } from "pinia";
import { ref } from "vue";
import { storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";
import { UserSelectDirItem } from "../types/Components.interface";

export const useUserSelectDirStore = defineStore("user-select-dir", () => {
    const userSelectDirs = ref<UserSelectDirItem[]>(
        storeGet<UserSelectDirItem[]>(StorageKey.user_select_dirs) ?? [],
    );

    const setUserSelectDirs = (dirs: UserSelectDirItem[]) => {
        userSelectDirs.value = dirs;
        storeSet(StorageKey.user_select_dirs, dirs);
    };

    return {
        userSelectDirs,
        setUserSelectDirs,
    };
});
