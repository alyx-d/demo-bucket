<script setup lang="ts">
import { message } from '@tauri-apps/plugin-dialog';
import { usePlayBottomStore } from '../store/PlayBottomStore';

const store = usePlayBottomStore();

const onButton1Click = () => {
    store.hide();
}

const onButton2Click = () => {
    store.show();
}

const onMessageClick = () => {
    message("Hello World");
}
</script>

<template>
    <div class="recommend">
        <span>Recommend</span>
        <div>{{ store.playBottonShow }}</div>
        <div class="test">
            <button type="button" @click="onButton1Click">hide</button>
            <button type="button" @click="onButton2Click">show</button>
            <button type="button" @click="onMessageClick">show</button>
        </div>
    </div>
</template>