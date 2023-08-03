<script setup lang="ts">
import Display from "./components/Display.vue";
import { listen, emit } from "@tauri-apps/api/event";
import { Parameter } from "./components/Parameters"
// var params: Parameter = new Parameter();
// params.listen();

</script>

<template>
    <div class="container">
        <div>
            <Display :width="10" :height="10"/>
        </div>
    </div>
</template>