
<script setup lang="ts">
import { ref } from "vue";
import { winConfig, Windows } from "./Window";
import { emit, listen } from "@tauri-apps/api/event";
import { parameterConfig } from "./Parameters"
import { invoke } from "@tauri-apps/api";

const displayWindow: winConfig = {
    label: 'playBoard',            // 窗口唯一label
    others: {
      title: 'minesweeper',              // 窗口标题
      url: '../../playboard.html', // 窗口加载的url
      width: 800,             // 窗口宽度
      height: 640,            // 窗口高度
      minWidth: 0,         // 窗口最小宽度
      minHeight: 0,        // 窗口最小高度
      center: true,           // 窗口居中显示
      resizable: false,        // 是否支持缩放
      maximized: false,       // 最大化窗口
      decorations: true,     // 窗口是否无边框及导航条
      alwaysOnTop: false,     // 置顶窗口
    }
}
const w = ref("");
const h = ref("");
const m = ref("");
const startDisabled = ref(false);

var windows: Windows = new Windows();
windows.listen();

async function start() {
  startDisabled.value = true;
  await invoke('start_game', {width: w.value, height: h.value, mines: m.value})
}
</script>

<template>
  <div class = "container" id="startarea">
    <div class = "container" id="inputarea">
      <p><input id="greet-input" v-model="w" placeholder="Width" /></p>
      <p><input id="greet-input" v-model="h" placeholder="height" /></p>
      <p><input id="greet-input" v-model="m" placeholder="number of mines" /></p>
    </div>
    <div id="clickarea">
      <button
        type="button"
        @click="start"
        :disabled="startDisabled" 
        >Start</button>
    </div>
    
  </div>

</template>

<style>
</style>