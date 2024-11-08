import { defineStore } from "pinia";
import { ref } from "vue";
import { storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";

export const usePlayBottomStore = defineStore("playBottom", () => {
    const playButtonShow = ref(
        !!storeGet<boolean>(StorageKey.play_bottom),
    );
    const title = ref(storeGet<string>(StorageKey.playing_title) ?? "");
    const hide = () => {
        playButtonShow.value = false;
        storeSet(StorageKey.play_bottom, false);
    };
    const show = () => {
        playButtonShow.value = true;
        storeSet(StorageKey.play_bottom, true);
    };
    const setTitle = (str: string) => {
        title.value = str;
        storeSet(StorageKey.playing_title, str);
    };
    return {
        playButtonShow,
        hide,
        show,
        title,
        setTitle,
    };
});
