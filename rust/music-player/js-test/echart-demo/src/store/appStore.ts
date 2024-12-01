import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";

export const useAppStore = defineStore("app", () => {
    const count = useLocalStorage("count", 0);
    const increment = () => count.value++;
    return { count, increment };
});