<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
const b1 = ref('');
const b2 = ref('');
const onButton1Click = async () => {
    b1.value = await invoke("button1");
}

const onButton2Click = async () => {
    b2.value = await invoke("button2");
}
</script>

<template>
    <div class="recommend">
        <span>Recommend</span>
        <div>{{ b1 }}</div>
        <div>{{ b2 }}</div>
        <div class="test">
            <button type="button" @click="onButton1Click">Button 1</button>
            <button type="button" @click="onButton2Click">Button 2</button>
        </div>
    </div>
</template>