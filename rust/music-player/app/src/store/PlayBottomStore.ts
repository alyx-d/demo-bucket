import { defineStore } from "pinia";
import { ref } from "vue";

export const usePlayBottomStore = defineStore("playBottom", () => {
    const playBottonShow = ref(false);
    const title = ref("");
    const hide = () => {
        playBottonShow.value = false;
    };
    const show = () => {
        playBottonShow.value = true;
    };
    const setTitle = (str: string) => {
        title.value = str;
    };
    return {
        playBottonShow,
        hide,
        show,
        title,
        setTitle,
    };
});
