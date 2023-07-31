
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { WebviewWindow } from "@tauri-apps/api/window";

/*
  Start.vue has Several missions:
  - Get the input from the user
  - Invoke the rust function to start the game
  - Pass width and height to Display.vue for display
  - Pop up a window/Replace current setting to show the result?
  - Wait for async GameResult from rust. Show Win/Lose and ask for restart.
*/

const name = ref("");
const w = ref("");
const h = ref("");
const m = ref("");

function start() {
  invoke("greet", { width: w , height: h, mines: m });
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
      <button type="button" @click="start" >Start</button>
    </div>
    
  </div>
  
  <!-- <div>{{ greetMsg }}</div> -->

</template>

<style>
</style>