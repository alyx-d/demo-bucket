<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { usePlayerStateStore } from '../store/PlayerStateStore';
import { invoke } from '@tauri-apps/api/core';
import Commands from '../common/Commands';
import { calculatePercent } from '../common/Utils';

const playerStore = usePlayerStateStore();

const process = ref(playerStore.playerVolume);

watch(() => playerStore.playerVolume, val => process.value = val);

const processVal = computed(() => {
    return process.value + "%";
});


const onProcessMouseDown = async (e: MouseEvent, el: HTMLElement) => {
    const trackEl = el.querySelector(".track") as HTMLElement;
    const offsetY = trackEl.getBoundingClientRect().bottom - e.clientY;
    const percentage = calculatePercent(offsetY, trackEl.offsetHeight);
    process.value = percentage;
    let volumn = Number((percentage / 100).toFixed(2));
    playerStore.setPlayerVolumn(process.value);
    const onMouseMove = (e: MouseEvent) => {
        const offsetY = trackEl.getBoundingClientRect().bottom - e.clientY;
        const percentage = calculatePercent(offsetY, trackEl.offsetHeight);
        process.value = percentage;
        volumn = Number((percentage / 100).toFixed(2));
        invoke(Commands.player_set_volume, { volume: volumn });
    };
    trackEl.addEventListener("mousemove", onMouseMove);
    const onMouseUp = (_: MouseEvent) => {
        trackEl.removeEventListener("mousemove", onMouseMove);
        trackEl.removeEventListener("mouseup", onMouseUp);
        trackEl.removeEventListener("mouseout", onMouseUp);
        playerStore.setPlayerVolumn(process.value);
        invoke(Commands.player_set_volume, { volume: volumn });
    };
    trackEl.addEventListener("mouseup", onMouseUp);
    trackEl.addEventListener("mouseout", onMouseUp);
};
</script>

<template>
    <div class="volumn-adjust">
        <div class="wrapper">
            <div class="slider" @mousedown="onProcessMouseDown($event, $el)">
                <div class="track">
                    <div class="process"></div>
                    <div class="ball"></div>
                </div>
            </div>
            <div class="title">
                <span class="text">{{ Math.round(process) + "%" }}</span>
            </div>
        </div>
        <div class="triangle"></div>
    </div>
</template>

<style scoped lang="css">
.volumn-adjust {
    --process: v-bind(processVal);

    width: 40px;
    height: 140px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    .wrapper {
        width: 100%;
        height: 100%;
        background-color: var(--volumn-adjust-bg-color);
        display: flex;
        flex-direction: column;
        align-items: center;
        border-radius: 6px;

        .slider {
            flex: 1;
            display: flex;
            justify-content: center;
            align-items: center;
            margin: 15px 0 5px 0;
            padding: 0 6px;
            cursor: pointer;

            .track {
                width: 7px;
                height: 100%;
                background-color: var(--volumn-adjust-track-bg-color);
                border-radius: 5px;
                position: relative;
                display: flex;
                align-items: flex-end;

                .process {
                    width: 100%;
                    border-radius: 5px;
                    background-color: var(--main-color);
                    height: var(--process);
                }

                .ball {
                    width: 12px;
                    height: 12px;
                    border-radius: 50%;
                    background-color: white;
                    position: absolute;
                    left: 50%;
                    transform: translateX(-50%) translateY(50%);
                    bottom: calc(var(--process));
                }
            }
        }

        .title {
            font-size: 10px;
            text-align: center;
            padding: 5px 0;
        }
    }

    .triangle {
        width: 15px;
        height: 15px;
        background-color: var(--volumn-adjust-bg-color);
        clip-path: polygon(50% 50%, 0 0, 100% 0);
        border-radius: 0 0 20px 20px;
        box-shadow: 5px 5px 5px var(--volumn-adjust-bg-color);
    }
}
</style>