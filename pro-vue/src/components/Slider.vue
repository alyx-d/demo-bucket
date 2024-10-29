<script setup lang="ts">
import {Ref, ref} from "vue";

const imgs: Ref<Array<string>> = ref([
  "https://ali-imgs.acfun.cn/kimg/EJvM1y8qPQoFYWNmdW4SBWltYWdlGi0xMjQxMDUwXzE5YWRkM2MxYzg2NzQzNDg5ODc4ZTNjNmJjNGM2NGJiLmpwZWc.webp?x-oss-process=image/resize,m_fill,w_964,h_494",
  "https://ali-imgs.acfun.cn/kimg/bs2/zt-image-host/ChgwOGVkZmNkZWFlMDUxMGQzZTFlYmEwMDcQm8zXLw.webp?x-oss-process=image/resize,m_fill,w_964,h_494",
  "https://ali-imgs.acfun.cn/kimg/bs2/zt-image-host/ChgwODljZmNkZWFlMDUxMGYwOTdlNGFiMDIQm8zXLw.webp?x-oss-process=image/resize,m_fill,w_964,h_494"
]);

const onPrevClick = () => {
  imgs.value.unshift(imgs.value.pop()!)
}
const onNextClick = () => {
  imgs.value.push(imgs.value.shift()!)
}
</script>

<template>
  <div class="operation">
    <button @click="onPrevClick">prev</button>
    <button @click="onNextClick">next</button>
  </div>
  <ul class="slider">
    <li v-for="(item, index) in imgs" :key="index">
      <img :key="index" :src="item" alt="img">
    </li>
  </ul>
</template>

<style scoped>
  .slider {
    list-style: none;
    li {
      &:not(:first-child) {
        display: none;
      }
      & {
        transition: all 1s ease-in-out;
      }
      img {
        width: 500px;
        height: 300px;
      }
    }
  }
</style>