<template>
  <div class="search-container">
    <div class="search-box">
      <input
        type="text"
        v-model="searchQuery"
        placeholder="Select a champion..."
        @input="onInput"
        @keydown.enter="onEnter"
        class="input"
        spellcheck="false"
        ref="searchInput"
      />
      <button @click="onSearch" class="search-button">Open Calculator</button>
    </div>
    <ul v-if="filteredChampions.length" class="suggestions">
      <li
        v-for="champion in filteredChampions"
        :key="champion.key"
        @click="selectChampion(champion)"
      >
        <img :src="champion.icon" alt="Champion Icon" class="icon" />
        {{ champion.name }}
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const searchQuery = ref('')
const champions = ref<any[]>([])
const filteredChampions = ref<any[]>([])
const selectedChampionKey = ref<string | null>(null)
const searchInput = ref<HTMLInputElement | null>(null)

onMounted(async () => {
  try {
    const response = await fetch('/champions.json')
    if (!response.ok) {
      throw new Error('Failed to load champions data')
    }
    const data = await response.json()
    champions.value = Object.keys(data).map((key) => ({
      key,
      name: data[key].name,
      icon: data[key].icon,
    }))
    if (searchInput.value) {
      searchInput.value.focus()
      searchInput.value.select()
    }
  } catch (error) {
    console.error('Error loading champions:', error)
  }
})

const onInput = () => {
  console.log('Current query:', searchQuery.value)
  filteredChampions.value = champions.value.filter(
    (champion) =>
      champion.key.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      champion.name.toLowerCase().includes(searchQuery.value.toLowerCase()),
  )
}

const onEnter = () => {
  if (filteredChampions.value.length > 0) {
    const champName = filteredChampions.value[0].key
    selectChampion(filteredChampions.value[0])
    onSearch(champName)
  }
}

const onSearch = async (key: string) => {
  if (key) {
    console.log('Opening calculator for:', searchQuery.value)
    try {
      const response = await fetch(`http://localhost:8080/setchampion/${key}`, {
        method: 'POST',
      })
      if (!response.ok) {
        throw new Error('Bad network response')
      }

      const result = await response.text()
      console.log('Response from server:', result)

      router.push('/calc')
    } catch (error) {
      console.error('Error:', error)
    }
  }
}

const selectChampion = (champion) => {
  searchQuery.value = champion.name
  selectedChampionKey.value = champion.key
  filteredChampions.value = []
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Rubik:wght@400;700&display=swap');

.search-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 80vh;
  margin-top: 3vh;
  position: relative;
}

.search-box {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 20px;
  width: 100%;
  max-width: 600px;
  padding: 0 10px;
}

.input {
  padding: 15px;
  border-radius: 15px;
  border: 1px solid #ccc;
  flex: 1;
  min-width: 200px;
  height: 5vh;
  font-size: 16px;
  font-family: 'Rubik', sans-serif;
  z-index: 1;
}

.search-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 15px 20px;
  border: none;
  border-radius: 15px;
  background-color: #732ec7;
  height: 5vh;
  min-height: 18px;
  color: white;
  cursor: pointer;
  font-size: 16px;
  font-family: 'Rubik', sans-serif;
  margin-left: 10px;
  z-index: 1;
}

.search-button:hover {
  background-color: #7930d1;
}

.suggestions {
  background: white;
  border: 1px solid #ccc;
  border-radius: 15px;
  width: 50vw;
  max-height: 200px;
  overflow-y: auto;
  list-style: none;
  padding: 0;
  margin: 5px 0 0 0;
  z-index: 10;
}

.suggestions li {
  display: flex;
  align-items: center;
  padding: 10px;
  cursor: pointer;
  color: #333;
}

.suggestions li:hover {
  background-color: #f0f0f0;
  color: #000;
}

.icon {
  width: 20px;
  height: 20px;
  margin-right: 10px;
}

</style>
