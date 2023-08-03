<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import Cell from "./Cell.vue";

var w = 0
var h = 0
console.log('js: Display.vue')
await listen("parameterInit", (event) => {
    console.log("js: parameter-init: " + event)
    var params = JSON.parse(event.payload)
    console.log("js: parameter_init: " + params.width)
    console.log("js: parameter_init: " + params.height)
    w = params.width
    h = params.height
})
const board: Ref<string[]> = ref([]);
const iter: Ref<string[][]> = ref([[]]);
async function read_into() {
  board.value = await invoke("display", {n: w!*h!});
  for (let i = 0; i < w!; i++) {
    iter.value[i] = []
    for (let j = 0; j < w!; j++) {
      iter.value[i][j] = board.value[i*w!+j]
      // iter.value[i].push(board.value[i*w+j]);
    }
  }
}
read_into();

function button_left_click(x: Number, y: Number): void {
  console.log("Button at "+x+", "+y+" is left clicked");
  read_into();
}
function button_right_click(x: Number, y: Number): void {
  console.log("Button at "+x+", "+y+" is right clicked");
  read_into();
}

</script>

<template>
  <div class="container">
    <div class="container" id="row" v-for="(line, row) in iter">
      <div class="container" id="col" v-for="(item, col) in line">
        <Cell 
          :button-style=item
          @left-click="button_left_click(row, col)"
          @right-click="button_right_click(row, col)"
        />
      </div>
    </div>
  </div>
</template>

<style>
.divclass {
  font-size: 10px;
}
</style>