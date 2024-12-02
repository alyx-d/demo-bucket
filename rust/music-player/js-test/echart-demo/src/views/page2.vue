<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as echarts from 'echarts';

const echartsContainer = ref<HTMLElement>();

onMounted(() => {
    // console.log('echartsContainer', echartsContainer.value);
    // return;
    const instance = echarts.init(echartsContainer.value, "dark");
    const option = {
        legend: {},
        tooltip: {},
        dataset: {
            source: [
                ['product', '2015', '2016', '2017'],
                ['Matcha Latte', 43.3, 85.8, 93.7],
                ['Milk Tea', 83.1, 73.4, 55.1],
                ['Cheese Cocoa', 86.4, 65.2, 82.5],
                ['Walnut Brownie', 72.4, 53.9, 39.1]
            ],
        },
        xAxis: {
            type: 'category',
        },
        yAxis: {},
        series: [
            { type: "bar" },
            { type: "bar" },
            { type: "bar" },
        ],
    };
    // setOption can use in async request to update data
    instance.setOption(option);
});
</script>

<template>
    <div class="page2">
        <h1>page2</h1>
        <div ref="echartsContainer" class="echarts-contariner"></div>
    </div>
</template>

<style scoped lang="css">
.page2 {
    .echarts-contariner {
        width: 100%;
        height: 300px;
    }
}
</style>