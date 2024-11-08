<script setup lang="ts">
import { computed } from 'vue';
import { usePlayBottomStore } from '../store/PlayBottomStore';

const store = usePlayBottomStore();

const playing = computed(() => {
    return true ? "playing" : "";
});
</script>

<template>
    <transition name="fade">
        <div class="play-bottom" v-if="store.playButtonShow">
            <div class="file-info">
                <div class="img-wrapper">
                    <img :class="playing" src="../assets/ac.jpg" alt="img" />
                </div>
                <div class="title">
                    <span>{{ store.title }}</span>
                </div>
            </div>
            <div class="operations"></div>
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

    .operations {
        width: 500px;
        height: 100%;
        background-color: red;
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