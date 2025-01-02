<template>
  <div class="champ-details" v-if="currentChampion">
    <div class="champion-card">
      <img :src="currentChampion.icon" :alt="currentChampion.name" class="champion-icon" />
      <div class="champion-info">{{ currentChampion.name }}, {{ currentChampion.title }}</div>
    </div>
  </div>
  <div v-else>
    <p>Loading champion details...</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const currentChampion = ref<any | null>(null)
const playerStats = new Map<string, number>

onMounted(async () => {
  try {
    const response = await fetch('http://localhost:8080/getchampion')
    if (!response.ok) {
      throw new Error('Failed to load champion data')
    }
    const data = await response.text()
    const championResponse = await fetch(`/champions.json`)
    if (!championResponse.ok) {
      throw new Error('Failed to load champions list')
    }
    const championsData = await championResponse.json()

    const championDetails = championsData[data]

    if (championDetails) {
      currentChampion.value = {
        name: championDetails.name,
        icon: championDetails.icon,
        title: championDetails.title,
      }
    } else {
      console.error('Champion not found in champions data')
    }
  } catch (error) {
    console.error('Error loading champion details:', error)
  }
})
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Rubik:wght@400;700&display=swap');
.champion-card {
  display: flex;
  align-items: center;
  margin: 10px;
}

.champion-icon {
  width: 100px;
  height: 100px;
  margin-right: 10px;
}

.champion-info {
  font-size: 3em;
  font-family: 'Rubik', sans-serif;
  font-weight: 500;
  color: #ffffff;
  margin: 0;
  line-height: 1.2;
}
</style>
