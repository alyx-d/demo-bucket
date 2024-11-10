<script setup lang="ts">
import { reactive } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { doScanDirs, usePlayerStateStore } from "../store/PlayerStateStore";
import { storeGet, storeSet } from "../common/Utils";
import StorageKey from "../common/StorageKey";

const playerStore = usePlayerStateStore();
const userSelectDirs = reactive<string[]>(playerStore.scanDirs.slice(2));
const userSelected = reactive<boolean[]>(
  storeGet<boolean[]>(StorageKey.user_select_dirs) ?? [true, true, ...Array.from({ length: userSelectDirs.length }, () => false)],
);


const addDir = async () => {
  const path = await open({
    directory: true, canCreateDirectories: true,
    title: "选择添加目录",
  });
  if (path) {

  }
};

const confirm = (parent: any) => {

};
</script>

<template>
  <div class="select-local-dir">
    <div class="wrapper">
      <div class="title">
        <h3>选择本地文件夹</h3>
      </div>
      <div class="tip">
        <span class="text">将自动扫描勾选的目录,文件增删实时同步</span>
      </div>
      <div class="form">
        <form>
          <div class="item">
            <input type="checkbox" id="my-music" name="my-music" v-model="userSelected[0]" />
            <label for="my-music">我的音乐</label>
          </div>
          <div class="item">
            <input type="checkbox" id="download" name="download" v-model="userSelected[1]" />
            <label for="download">下载</label>
          </div>
          <div class="item" v-for="(item, index) in userSelectDirs" :key="index">
            <input type="checkbox" :id="`usd-${index}`" :name="`usd-${index}`" v-model="userSelected[index + 2]" />
            <label :for="`usd-${index}`">{{ item }}</label>
          </div>
        </form>
      </div>
      <div class="button-group">
        <button @click="addDir" class="button add-dir">添加文件夹</button>
        <button @click="confirm($parent)" class="button confirm">确认</button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="css">
.select-local-dir {
  width: 350px;
  height: 320px;

  .wrapper {
    height: 100%;
    display: flex;
    flex-direction: column;
  }

  .title {
    text-align: center;
    margin: 20px 0;
  }

  .tip {
    font-size: 13px;
  }

  .form {
    overflow-y: auto;

    .item {
      margin: 20px 0;
      font-size: 13px;
      display: flex;
      align-items: center;
      width: max-content;

      &:hover {
        cursor: pointer;
      }

      input[type="checkbox"] {
        margin-right: 10px;
        cursor: pointer;
        display: none;

        &+label {
          cursor: pointer;
          position: relative;
          padding-left: 25px;

        }

        &+label::before {
          content: "";
          background-color: white;
          position: absolute;
          left: 0;
          top: 0;
          width: 15px;
          height: 15px;
          border: 1px solid #ccc;
          border-radius: 4px;
        }

        &:not(:checked)+label:hover::before {
          border-color: #999;
        }

        &:checked+label::before {
          content: "\2714";
          background-color: #f6142e;
          display: flex;
          justify-content: center;
          align-items: center;
          color: white;
        }
      }

    }
  }

  .button-group {
    margin-top: auto;

    .button {
      width: 150px;
      height: 40px;
      border-radius: 20px;
      font-size: 14px;
      background-color: #fff;
      border: 1px solid #ccc;

      &:not(:last-child) {
        margin-right: 10px;
      }

      &.confirm {
        background-color: var(--main-color);
        color: white;

        &:hover {
          background-color: var(--main-color-dark);
        }
      }

      &.add-dir {
        &:hover {
          background-color: #eeeeee60;
          border-color: #999;
        }
      }

      &:hover {
        cursor: pointer;
      }
    }

  }
}
</style>