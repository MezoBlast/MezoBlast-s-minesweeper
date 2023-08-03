<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Start from "./components/Start.vue";
import Testboard from "./components/Testboard.vue";
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>
      <div>
        <Start />
      </div>
      <div>
      <p>From App.vue</p>
      <Suspense>
        <Testboard listen_method="parameterInit"/>
      </Suspense>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #74a7ff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
