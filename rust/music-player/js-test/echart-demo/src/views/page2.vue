<script setup lang="ts">
import { onMounted, ref } from 'vue';
import * as echarts from 'echarts';

const echartsContainer = ref<any>();

onMounted(() => {
    // console.log('echartsContainer', echartsContainer.value);
    // return;
    const instance = echarts.init(echartsContainer.value);
    const option = {
        title: {
            text: '某站点用户访问来源',
            subtext: '纯属虚构',
            x: 'center'
        },
        tooltip: {
            trigger: 'item',
            formatter: "{a} <br/>{b} : {c} ({d}%)"
        },
        legend: {
            orient: 'vertical',
            left: 'left',
            data: ['直接访问', '邮件营销', '联盟广告', '视频广告', '搜索引擎']
        },
        series: [
            {
                name: '访问来源',
                type: 'pie',
                radius: '55%',
                center: ['50%', '60%'],
                data: [
                    { value: 335, name: '直接访问' },
                    { value: 310, name: '邮件营销' },
                    { value: 234, name: '联盟广告' },
                    { value: 135, name: '视频广告' },
                    { value: 1548, name: '搜索引擎' }
                ],
                emphasis: {
                    itemStyle: {
                        shadowBlur: 10,
                        shadowOffsetX: 0,
                        shadowColor: 'rgba(0, 0, 0, 0.5)'
                    }
                }
            }
        ]
    };
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