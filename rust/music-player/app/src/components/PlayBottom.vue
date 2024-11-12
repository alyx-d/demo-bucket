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
import VolumnAdjust from './VolumnAdjust.vue';

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
    store.show();
});

listen(PlayerEvents.Pause, (event) => {
    const index = event.payload as number;
    playerStore.setPlaying(false, index, true);
    playerStore.setPause(true);
});

listen(PlayerEvents.Resume, (event) => {
    const index = event.payload as number;
    playerStore.setPlaying(true, index, true);
    playerStore.setPause(false);
});

const isSilence = computed(() => {
    return playerStore.playerVolumn == 0;
});

let playerVolumn = 100;
const setSilense = async () => {
    if (isSilence.value) {
        playerStore.setPlayerVolumn(playerVolumn);
    } else {
        playerVolumn = playerStore.playerVolumn;
        playerStore.setPlayerVolumn(0);
    }
    invoke(Commands.player_set_volume, { volume: Number((playerStore.playerVolumn / 100).toFixed(2)) });
};
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
            <div class="button-group">
                <div class="volumn">
                    <svg @click="setSilense" v-if="!isSilence" t="1731332608984" class="icon" viewBox="0 0 1024 1024"
                        version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4259" data-darkreader-inline-fill=""
                        width="200" height="200">
                        <path class="path"
                            d="M255.44704 326.51264l217.2928-134.12352a54.1696 54.1696 0 0 1 74.09664 16.896c5.3248 8.43776 8.17152 18.18624 8.17152 28.11904v560.45568c0 29.32736-24.08448 53.08416-53.76 53.08416-10.07616 0-19.94752-2.78528-28.48768-8.06912l-217.2928-134.144H157.16352C115.6096 708.75136 81.92 675.49184 81.92 634.42944V400.83456c0-41.04192 33.6896-74.32192 75.264-74.32192h98.26304z m26.35776 58.85952a32.5632 32.5632 0 0 1-17.1008 4.85376h-107.52a10.69056 10.69056 0 0 0-10.752 10.60864v233.59488c0 5.85728 4.8128 10.62912 10.752 10.62912h107.52c6.0416 0 11.96032 1.67936 17.1008 4.83328l208.6912 128.79872V256.57344l-208.6912 128.8192zM803.2256 837.8368a32.54272 32.54272 0 0 1-45.60896-0.77824 31.55968 31.55968 0 0 1 0.75776-45.056c158.9248-151.6544 158.9248-397.1072 0-548.7616a31.58016 31.58016 0 0 1-0.75776-45.056 32.5632 32.5632 0 0 1 45.60896-0.75776c185.1392 176.70144 185.1392 463.68768 0 640.4096z m-107.76576-127.20128a32.5632 32.5632 0 0 1-45.60896-1.2288 31.55968 31.55968 0 0 1 1.2288-45.03552c86.9376-81.24416 86.9376-212.23424 0-293.49888a31.60064 31.60064 0 0 1-1.2288-45.03552 32.5632 32.5632 0 0 1 45.60896-1.2288c113.78688 106.3936 113.78688 279.63392 0 386.048v-0.02048z"
                            fill="#000000" p-id="4260" data-darkreader-inline-fill="" style="#000000"></path>
                    </svg>
                    <svg @click="setSilense" v-if="isSilence" t="1731388941324" class="icon" viewBox="0 0 1024 1024"
                        version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="4753" width="200" height="200">
                        <path class="path"
                            d="M448 272.1v479.8l-130.5-76.7c-39.3-23.1-84.2-35.3-129.8-35.3H128V384.1h59.7c45.6 0 90.5-12.2 129.8-35.3L448 272.1m-0.1-64.1c-10.8 0-21.9 2.8-32.3 8.9l-130.5 76.7c-29.5 17.3-63.1 26.5-97.3 26.5H128c-35.3 0-64 28.6-64 64v255.8c0 35.3 28.7 64 64 64h59.7c34.2 0 67.8 9.1 97.3 26.5l130.5 76.7c10.4 6.1 21.5 8.9 32.3 8.9 33.4 0 64.1-26.7 64.1-64.1V272.1c0.1-37.4-30.6-64.1-64-64.1zM837.3 512l113.4-113.4c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L792 466.7 678.6 353.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L746.7 512 633.4 625.4c-12.5 12.5-12.5 32.8 0 45.3 6.2 6.2 14.4 9.4 22.6 9.4s16.4-3.1 22.6-9.4L792 557.3l113.4 113.4c6.2 6.2 14.4 9.4 22.6 9.4s16.4-3.1 22.6-9.4c12.5-12.5 12.5-32.8 0-45.3L837.3 512z"
                            p-id="4754"></path>
                    </svg>
                    <VolumnAdjust class="volume-adjust" />
                </div>
            </div>
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
    border-top: 1px solid var(--bottom-bar-border-color);


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
        height: 100%;
        display: flex;
        justify-content: flex-end;
        align-items: center;

        .volumn {
            display: flex;
            justify-content: center;
            align-items: center;
            position: relative;

            .icon {
                width: 20px;
                height: 20px;
                cursor: pointer;

                .path {
                    fill: var(--icon-path-fill-color);
                }

                &:hover .path {
                    fill: var(--icon-path-hover-fill-color);
                }
            }

            &:hover .volumn-adjust {
                transform: scaleY(1);
                transition: 0.1s;
            }

            .volumn-adjust {
                position: absolute;
                bottom: 100%;
                transform: scaleY(0) translateY(80%);
            }
        }
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