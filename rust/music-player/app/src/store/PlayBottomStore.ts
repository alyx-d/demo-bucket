import { defineStore } from "pinia";
import { ref } from "vue";
import { storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";

export const usePlayBottomStore = defineStore("playBottom", () => {
    const playBottonShow = ref(
        !!storeGet<boolean>(StorageKey.play_bottom),
    );
    const title = ref(storeGet<string>(StorageKey.playing_title) ?? "");
    const hide = () => {
        playBottonShow.value = false;
        storeSet(StorageKey.play_bottom, false);
    };
    const show = () => {
        playBottonShow.value = true;
        storeSet(StorageKey.play_bottom, true);
    };
    const setTitle = (str: string) => {
        title.value = str;
        storeSet(StorageKey.playing_title, str);
    };
    return {
        playBottonShow,
        hide,
        show,
        title,
        setTitle,
    };
});
