<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Cell from "./Cell.vue";


const props = defineProps(
  {
    width: Number,
    height: Number,
  }
)
const board: Ref<string[]> = ref([]);
const iter: Ref<string[][]> = ref([[]]);
async function read_into() {
  board.value = await invoke("display", {n: props.width!*props.height!});
  for (let i = 0; i < props.width!; i++) {
    iter.value[i] = []
    for (let j = 0; j < props.width!; j++) {
      iter.value[i][j] = board.value[i*props.width!+j]
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