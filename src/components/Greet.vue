<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";

const greetMsg = ref("");
const name = ref("");
const w = ref();
const h = ref();
const m = ref();

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
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
      <button type="button" @click="greet" >Greet</button>
    </div>
    
  </div>
  
  <!-- <div>{{ greetMsg }}</div> -->

</template>

<style>
/* #startarea{
  box-sizing: border-box;
  -moz-box-sizing: border-box;
}
#inputarea{
  float: left;
  display: inline;
}
#clickarea{
  float: right;
  display: inline;
} */
</style>