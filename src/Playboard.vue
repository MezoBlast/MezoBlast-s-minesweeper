<script setup lang="ts">
import Display from "./components/Display.vue";
import { listen, emit } from "@tauri-apps/api/event";
import { Parameter } from "./components/Parameters"

</script>

<template>
    <div class="container">
        <div>
            <suspense>
                <Display/>
            </suspense>
        </div>
    </div>
</template>