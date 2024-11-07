<script setup lang="ts">
import {ref} from "vue";

const showState = ref(false);

const close = () => {
  showState.value = false;
}

const open = () => {
  showState.value = true;
}

defineExpose<ExposeMethods>({
  open, close
});

export interface ExposeMethods {
  open: () => void,
  close: () => void,
}
</script>

<template>
  <teleport to="body">
    <div class="dialog" v-if="showState">
      <div class="modal">
        <div class="slot">
          <img class="icon_close" alt="" src="/icons/icon_close.svg" @click="close"/>
          <slot>
            <div class="default">
              <span>默认slot</span>
            </div>
          </slot>
        </div>
      </div>
    </div>
  </teleport>
</template>

<style scoped lang="css">
.dialog {
  .modal {
    width: 100%;
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 100;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .slot {
    z-index: 101;
    padding: 40px;
    position: relative;
    background-color: #fff;
    border-radius: 10px;

    .icon_close {
      width: 20px;
      height: 20px;
      position: absolute;
      top: 20px;
      right: 20px;
      cursor: pointer;
      transition: transform 0.3s ease-in-out;

      &:hover {
        transform: rotate(90deg);
      }
    }

    .default {
      width: 250px;
      height: 250px;
      display: flex;
      justify-content: center;
      align-items: center;
    }
  }
}
</style>