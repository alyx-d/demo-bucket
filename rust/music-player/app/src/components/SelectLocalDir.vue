<script setup lang="ts">
import { ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { usePlayerStateStore } from "../store/PlayerStateStore";
import { UserSelectDirItem } from "../types/Components.interface";
import { useUserSelectDirStore } from "../store/UserSelectDirStore";

const playerStore = usePlayerStateStore();
const userSelectDirStore = useUserSelectDirStore();
const userSelectDirs = ref<UserSelectDirItem[]>(userSelectDirStore.userSelectDirs);

watch(() => userSelectDirStore.userSelectDirs, val => {
  userSelectDirs.value = val;
});


const addDir = async () => {
  const path = await open({
    directory: true, canCreateDirectories: true,
    title: "选择添加目录",
  });
  if (path) {
    if (userSelectDirs.value.some((item) => item.path == path)) {
      alert("该目录已存在");
    } else {
      userSelectDirStore.setUserSelectDirs([
        ...userSelectDirs.value,
        {
          path: path,
          checked: false,
        },
      ]);
    }
  }
};

const confirm = (parent: any) => {
  userSelectDirStore.setUserSelectDirs(userSelectDirs.value);
  const dirs = userSelectDirs.value.filter((item) => item.checked);
  playerStore.setScanDirs(dirs.map((item) => item.path));
  parent?.close();
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
            <input type="checkbox" id="my-music" name="my-music" v-model="userSelectDirs[0].checked" />
            <label for="my-music">我的音乐</label>
          </div>
          <div class="item">
            <input type="checkbox" id="download" name="download" v-model="userSelectDirs[1].checked" />
            <label for="download">下载</label>
          </div>
          <div class="item" v-for="(item, index) in userSelectDirs.slice(2)" :key="index">
            <input type="checkbox" :id="`${item.path}`" :name="`${item.path}`"
              v-model="userSelectDirs[index + 2].checked" />
            <label :for="`${item.path}`">{{ item.path }}</label>
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
          background-color: var(--dialog-bg-color);
          position: absolute;
          left: 0;
          top: 0;
          width: 15px;
          height: 15px;
          border: 1px solid var(--dialog-bg-color);
          border-radius: 4px;
        }

        &:not(:checked)+label::before {
          border-color: var(--checkbox-border-color);
        }

        &:not(:checked)+label:hover::before {
          border-color: var(--checkbox-hover-border-color);
        }

        &:checked+label::before {
          content: "\2714";
          background-color: #f6142e;
          display: flex;
          justify-content: center;
          align-items: center;
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
      background-color: var(--dialog-bg-color);
      border: 1px solid #ddd;

      &:not(:last-child) {
        margin-right: 10px;
      }

      &.confirm {
        color: var(--select-dir-confirm-button-text-color);
        background-color: var(--main-color);

        &:hover {
          background-color: var(--main-color-dark);
        }
      }

      &.add-dir {
        color: var(--select-dir-button-text-color);

        &:hover {
          background-color: var(--btn-hover-bg-color);
          border-color: #ddd;
        }
      }

      &:hover {
        cursor: pointer;
      }
    }

  }
}
</style>