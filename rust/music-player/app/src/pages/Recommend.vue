<script setup lang="ts">
import { usePlayBottomStore } from '../store/PlayBottomStore';

const store = usePlayBottomStore();

const onButton1Click = () => {
    store.hide();
}

const onButton2Click = () => {
    store.show();
}
</script>

<template>
    <div class="recommend">
        <span>Recommend</span>
        <div>{{ store.playButtonShow }}</div>
        <div class="test">
            <button type="button" @click="onButton1Click">hide</button>
            <button type="button" @click="onButton2Click">show</button>
        </div>
    </div>
</template>