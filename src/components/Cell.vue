<script setup lang="ts">
import { E } from "@tauri-apps/api/path-c062430b";
import { ref } from "vue";

const bomb = "&#128163";
const flag = "&#128681";
const content = ref("");
const width = ref(20);
const height = ref(20);

function click(event: MouseEvent) {
  content.value = bomb;
}
function rightclick(event: MouseEvent) {
  content.value = flag;
}
</script>

<template>
  <button id="cellbutton" @click.prevent="click" @auxclick.prevent="rightclick" @contextmenu.prevent v-html="content">
  </button>
</template>

<style>
#row {
  display: flex;
  height: 20px;
  flex-direction: row;
  /* justify-content: center; */
}

#col {
  display: flex;
  width: 20px;
  flex-direction: column;
}

#cellbutton {
  width: 20px;
  height: 20px;
  padding: 0px 0px;
  border-radius: 0px;
  font: inherit;
  font-size: 10px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  text-size-adjust: 10%;
  text-align: center;
  vertical-align: middle;
  line-height: 10px;
}
#cellbutton:active {
  background-color: #949494;
}
#cellbutton:disabled {
  background-color: #949494;
}
</style>
