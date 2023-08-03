
<script setup lang="ts">
import { ref } from "vue";
import { winConfig, Windows } from "./Window";
import { emit, listen } from "@tauri-apps/api/event";
import { parameterConfig } from "./Parameters"
import { invoke } from "@tauri-apps/api";
import Testboard from "./Testboard.vue";

const w = ref("");
const h = ref("");
const m = ref("");
const startDisabled = ref(false);

async function start() {
  startDisabled.value = false;
  await invoke('start_game', {width: w.value, height: h.value, mines: m.value}).catch((err) => {
    console.log(err);
  });
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
    <div>
      <p>From Start.vue</p>
      <Suspense>
        <Testboard listen_method="parameterInit"/>
      </Suspense>
    </div>
  </div>

</template>

<style>
</style>