<script setup lang="ts">
import Display from "./components/Display.vue";
import { listen, emit } from "@tauri-apps/api/event";
import { Parameter } from "./components/Parameters"
var params: Parameter = new Parameter();
params.listen();

</script>

<template>
    <div class="container">
        <div>
            <Display :width="params.width" :height="params.height"/>
        </div>
    </div>
</template>