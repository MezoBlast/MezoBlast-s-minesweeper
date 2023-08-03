<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
var msg = "I'm listening"
const props = defineProps({
    listen_method: String,
})

await listen(props.listen_method, (event) => {
    msg = "I hear you."
})
</script>
<template>
{{ msg }}
</template>