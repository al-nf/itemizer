<template>
  <div id="app" class="background">
    <main>
      <router-view></router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
html, body {
  height: 100%;
  margin: 0;
  background-image: url('@/assets/background.jpg');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
}

main {
  padding: 2vh 2vw;
  z-index: 2;
  flex-grow: 1;
}
</style>
