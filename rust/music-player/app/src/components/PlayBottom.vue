<script setup lang="ts">
import { computed } from 'vue';
import { usePlayBottomStore } from '../store/PlayBottomStore';
import { usePlayerStateStore } from '../store/PlayerStateStore';
import ProcessBar from './ProcessBar.vue';
import { durationToSecs, secsToDuration } from '../common/Utils';
import { invoke } from '@tauri-apps/api/core';
import Commands from '../common/Commands';
import { listen } from '@tauri-apps/api/event';
import PlayerEvents from '../common/PlayerEvents';

const store = usePlayBottomStore();
const playerStore = usePlayerStateStore();

const paused = computed(() => {
    return playerStore.isPlaying ? "" : "paused";
});

const onPreviousClick = async () => {
    invoke(Commands.player_prev);
};

const onPlayOrPauseClick = async () => {
    if (playerStore.isPlaying) {
        invoke(Commands.player_pause);
    } else {
        playerStore.setPlaying(true, playerStore.playingIndex, true);
        invoke(Commands.player_resume);
    }
};

const onNextClick = async () => {
    invoke(Commands.player_next);
};

listen(PlayerEvents.Play, (event) => {
    const index = event.payload as number;
    playerStore.setPlaying(true, index);
    playerStore.setTotalDuration(durationToSecs(playerStore.playList[index].totalDuration));
    store.setTitle(playerStore.playList[index].title);
});

listen(PlayerEvents.Pause, (event) => {
    const index = event.payload as number;
    playerStore.setPlaying(false, index, true);
    playerStore.setPause(true);
});

listen(PlayerEvents.Resume, (event) => {
    const index = event.payload as number;
    playerStore.setPlaying(true, index);
    playerStore.setPause(false);
});

</script>

<template>
    <transition name="fade">
        <div class="play-bottom" v-if="store.playBottonShow">
            <div class="file-info">
                <div class="img-wrapper">
                    <img :class="`playing ${paused}`" src="../assets/ac.jpg" alt="img" />
                </div>
                <div class="title">
                    <span>{{ store.title }}</span>
                </div>
            </div>
            <div class="operation-group">
                <div class="operations">
                    <div class="img-wrapper previous" @click="onPreviousClick">
                        <img class="icon" src="/icons/previous.svg" alt="previous" />
                    </div>
                    <div class="img-wrapper play" @click="onPlayOrPauseClick">
                        <img class="icon" src="/icons/play_fill_white.svg" alt="play" v-if="!playerStore.isPlaying" />
                        <img class="icon" src="/icons/pause_white.svg" alt="play" v-if="playerStore.isPlaying" />
                    </div>
                    <div class="img-wrapper next" @click="onNextClick">
                        <img class="icon" src="/icons/next.svg" alt="next" />
                    </div>
                </div>
                <div class="process">
                    <div class="current-duration">
                        <span>{{ secsToDuration(playerStore.playingPos) }}</span>
                    </div>
                    <ProcessBar :process="playerStore.playingPos" class="process-bar" />
                    <div class="total-duration">
                        <span>{{ secsToDuration(playerStore.totalDuration) }}</span>
                    </div>
                </div>
            </div>
            <div class="button-group"></div>
        </div>
    </transition>
</template>

<style scoped lang="css">
.fade-enter-active,
.fade-leave-active {
    transition: all var(--bottom-bar-duration) ease-in;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
    transform: translateY(100%);
    will-change: transform;
}

.play-bottom {
    display: flex;
    align-items: center;
    justify-content: space-around;


    .file-info {
        flex: 1;
        display: flex;
        justify-content: center;

        .img-wrapper {
            width: 50px;
            height: 50px;

            img {
                width: 100%;
                height: 100%;
                border-radius: 50%;

                &.playing {
                    animation: spin 5s linear infinite;

                    &.paused {
                        animation-play-state: paused;
                    }
                }
            }
        }

        .title {
            flex: 1;
            padding-left: 10px;
            display: flex;
            align-items: center;
            font-size: 13px;
        }
    }

    .file-info,
    .button-group {
        margin: 10px 30px;
    }

    .operation-group {
        width: 500px;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        .operations {
            display: flex;
            justify-content: center;
            align-items: center;

            .img-wrapper {
                width: 25px;
                height: 25px;
                cursor: pointer;
                opacity: 0.9;

                &:not(:last-child) {
                    margin-right: 25px;
                }

                &.play {
                    width: 40px;
                    height: 40px;
                    border-radius: 50%;
                    background-color: var(--main-color);

                    & img {
                        padding: 10px;

                    }

                    &:hover {
                        opacity: 1;
                        transform: scale(1.05);
                    }
                }

                img {
                    width: 100%;
                    height: 100%;
                    opacity: 0.9;

                    &:hover {
                        opacity: 1;
                        transform: scale(1.05);
                    }
                }
            }
        }

        .process {
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 9px;
            color: #777;

            .process-bar {
                width: 250px;
            }
        }
    }

    .button-group {
        flex: 1;
    }


}

@keyframes spin {
    0% {
        transform: rotate(0deg);
    }

    100% {
        transform: rotate(360deg);
    }
}
</style>