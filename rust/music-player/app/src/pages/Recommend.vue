<script setup lang="ts">
const onToogleColorScheme = async (scheme: "dark" | "light" | "auto") => {
    if (scheme === "auto") {
        if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
            document.documentElement.setAttribute("data-theme", "dark");
        } else {
            document.documentElement.setAttribute("data-theme", "light");
        }
        document.documentElement.setAttribute("auto", "auto");
    } else {
        document.documentElement.setAttribute("data-theme", scheme);
        document.documentElement.setAttribute("auto", "");
    }
};

</script>

<template>
    <div class="recommend">
        <span>Recommend</span>
        <div class="test">
            <button type="button" @click="onToogleColorScheme('dark')">dark</button>
            <button type="button" @click="onToogleColorScheme('light')">light</button>
            <button type="button" @click="onToogleColorScheme('auto')">auto</button>
        </div>
    </div>
</template>