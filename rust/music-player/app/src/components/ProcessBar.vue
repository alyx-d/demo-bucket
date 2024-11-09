<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { usePlayerStateStore } from '../store/PlayerStateStore';
import { invoke } from '@tauri-apps/api/core';
import Commands from '../common/Commands';

const playerStore = usePlayerStateStore();

const pos = playerStore.playingPos;
let totalDuration = playerStore.totalDuration;
watch(() => playerStore.totalDuration, val => {
    totalDuration = val;
});

const calculatePercent = (p: number, t: number) => {
    return Math.min(Math.max((p / t) * 100, 0), 100);
};

const process = ref(totalDuration > 0 ? calculatePercent(pos, totalDuration) : 0);
watch(() => playerStore.playingPos, val => {
    process.value = calculatePercent(val, totalDuration);
});

const processVal = computed(() => {
    return process.value + "%";
});

const onTrackMouseDown = async (e: MouseEvent, el: HTMLElement) => {
    const trackEl = el.querySelector(".track") as HTMLElement;
    const ballEl = el.querySelector(".ball") as HTMLElement;
    const trackWidth = trackEl.offsetWidth;
    const offsetX = e.clientX - trackEl.getBoundingClientRect().left;
    const percentage = calculatePercent(offsetX, trackWidth);
    process.value = percentage;
    const pos = Math.round(percentage * totalDuration / 100);
    playerStore.setPlayingPos(pos);
    invoke(Commands.player_seek, { pos });
    const onTrackMouseMove = (e: MouseEvent) => {
        const offsetX = e.clientX - trackEl.getBoundingClientRect().left;
        const percentage = calculatePercent(offsetX, trackWidth);
        process.value = percentage;
        const pos = Math.round(percentage * totalDuration / 100);
        playerStore.setPlayingPos(pos);
        ballEl.style.display = "block";
    };
    document.addEventListener("mousemove", onTrackMouseMove);
    const onMouseUp = (_: MouseEvent) => {
        document.removeEventListener("mousemove", onTrackMouseMove);
        document.removeEventListener("mouseup", onMouseUp);
        invoke(Commands.player_seek, { pos: playerStore.playingPos });
    };
    document.addEventListener("mouseup", onMouseUp);
};
</script>
<template>
    <div class="process-bar" @mousedown="onTrackMouseDown($event, $el)">
        <div class="track">
            <div class="process"></div>
            <div class="ball"></div>
        </div>
    </div>
</template>

<style scoped lang="css">
.process-bar {
    --process: v-bind(processVal);

    width: 100%;
    height: 10px;
    padding: 10px 0;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;

    &:hover .track .ball {
        display: block;
    }

    .track {
        height: 5px;
        width: 92%;
        border-radius: 5px;
        background-color: #ccc;
        position: relative;

        .process {
            height: 5px;
            border-radius: 5px;
            background-color: var(--main-color);
            width: var(--process);
        }

        .ball {
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background-color: white;
            position: absolute;
            top: 50%;
            transform: translateY(-50%);
            left: calc(var(--process) - 1%);
            display: none;
        }
    }
}
</style>