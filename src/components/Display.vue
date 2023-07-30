<script setup lang="ts">
import { ref } from "vue";
import Cell from "./Cell.vue";
const bomb = "&#128163"
var iter: number[][] = init();

function init(): number[][] {
  var iter: number[][] = [];
  for (let i = 0; i < 20; i++) {
    iter.push([]);
    for (let j = 0; j < 20; j++) {
      iter[i].push(0);
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