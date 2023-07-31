<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Cell from "./Cell.vue";

/*  
  Display component has three missions:
  1. After start, initialize and display the board
  2. After click/right click, invoke the rust function to update game
  3. After update, display the board
  
  Current thoughts on API：
  - Board is a 2D array of Cell (from Cell.vue)
  - Each cell contains a cellbutton (see Cell.vue))
  - Cell is in charge of click and right click
  - Cell is also in charge of displaying the content with **emoji**
*/
var iter: number[][] = init();

function init(): number[][] {
  var original: number[] = invoke("display", {n: 400});
  var iter: number[][] = [[]] ;
  for (let i = 0; i < 20; i++) {
    iter.push([]);
    for (let j = 0; j < 20; j++) {
      iter[i].push(original[i*20+j]);
    }
  }
  return iter;
}

</script>

<template>
  <div class="container">
    <div class="container" id="row" v-for="item in iter">
      <div class="container" id="col" v-for="item in iter">
        <Cell />
      </div>
    </div>
  </div>
</template>

<style>
.divclass {
  font-size: 10px;
}
</style>