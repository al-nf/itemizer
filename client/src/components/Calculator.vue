<template>
  <div class="search-container">
    <div class="champ-details" v-if="currentChampion">
      <div class="champion-card">
        <img :src="currentChampion.icon" :alt="currentChampion.name" class="champion-icon" />
        <div class="champion-info">{{ currentChampion.name }}, {{ currentChampion.title }}</div>
      </div>
    </div>
    <div v-else>
      <p>Loading champion details...</p>
    </div>

    <div class="search-box">
      <input
        type="text"
        v-model="searchQuery"
        placeholder="Select an item..."
        @input="onInput"
        @keydown.enter="onEnter"
        class="input"
        spellcheck="false"
        ref="searchInput"
      />
      <button @click="onSearch(searchQuery)" class="search-button">Add Item</button>
    </div>

    <ul v-if="filteredItems.length" class="suggestions">
      <li
        v-for="item in filteredItems"
        :key="item.key"
        @click="selectItem(item)"
      >
        <img :src="item.icon" alt="Item Icon" class="icon" />
        {{ item.name }}
      </li>
    </ul>

    <div class="item-details" v-if="currentItem">
      <div class="item-card">
        <img :src="currentItem.icon" :alt="currentItem.name" class="item-icon" />
        <div class="item-info">{{ currentItem.name }}</div>
      </div>
    </div>
    <div v-else>
      <p>Loading item details...</p>
    </div>
  </div>
</template>


<script setup lang="ts">
import { ref, onMounted } from 'vue'

const currentItem = ref<any | null>(null)
const currentChampion = ref<any | null>(null)
const items = ref<any[]>([])
const champions = ref<any[]>([])
const searchQuery = ref('')
const filteredItems = ref<any[]>([])
const addedItems = ref<any[]>([]); // Holds added items

onMounted(async () => {
  try {
    const response = await fetch('http://localhost:8080/item')
    if (!response.ok) {
      throw new Error('Failed to load items data')
    }
    const data = await response.json()
    items.value = Object.keys(data).map((key) => ({
      key,
      name: data[key].name,
      icon: data[key].icon,
    }))
  } catch (error) {
    console.error('Error loading items:', error)
  }

  await fetchChampionDetails()
})

const fetchChampionDetails = async () => {
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
}

const onInput = () => {
  filteredItems.value = items.value.filter((item) =>
    item.name.toLowerCase().includes(searchQuery.value.toLowerCase()),
  )
}

const onEnter = () => {
  if (filteredItems.value.length > 0) {
    const itemName = filteredItems.value[0].name
    selectItem(filteredItems.value[0])
  }
}

const onSearch = async (name: string) => {
  if (name) {
    try {
      const response = await fetch(`http://localhost:8080/additem/${name}`, {
        method: 'POST',
      })
      if (!response.ok) {
        throw new Error('Bad network response')
      }

      const result = await response.text()
      console.log('Response from server:', result)

      if (addedItems.value.length < 6) {
        const itemToAdd = items.value.find((item) => item.name === name);
        if (itemToAdd) {
          addedItems.value.push(itemToAdd);
        }
      }

    } catch (error) {
      console.error('Error:', error)
    }
    searchQuery.value = ''
    currentItem.value = null
  }
}

const selectItem = (item) => {
  searchQuery.value = item.name
  currentItem.value = item
  filteredItems.value = []
  onSearch(item.name)
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Fira Sans:wght@400;700&display=swap');

.search-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  height: auto;
  margin-top: 3vh;
  position: relative;
}

.champ-details {
  margin-bottom: 20px;
  width: 100%;
  text-align: center;
  padding: 20px;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}

.search-box {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 20px;
  width: 100%;
  max-width: 400px;
  padding: 0 10px;
}
.input {
  padding: 10px;
  border-radius: 10px;
  border: 1px solid #ccc;
  flex: 1;
  min-width: 150px;
  height: 40px;
  font-size: 14px;
  font-family: 'Fira Sans', sans-serif;
  z-index: 1;
}

.search-button {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 15px;
  border: none;
  border-radius: 10px;
  background-color: #732ec7;
  height: 40px;
  min-height: 18px;
  color: white;
  cursor: pointer;
  font-size: 14px;
  font-family: 'Fira Sans', sans-serif;
  margin-left: 10px;
  z-index: 1;
}

.search-button:hover {
  background-color: #7930d1;
}

.suggestions {
  background: white;
  border: 1px solid #ccc;
  border-radius: 10px;
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

.item-card {
  display: flex;
  align-items: center;
  margin: 10px;
}

.item-icon {
  width: 100px;
  height: 100px;
  margin-right: 10px;
}

.item-info {
  font-size: 2em;
  font-family: 'Fira Sans', sans-serif;
  font-weight: 500;
  color: #ffffff;
  margin: 0;
  line-height: 1.2;
}

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
  font-family: 'Fira Sans', sans-serif;
  font-weight: 500;
  color: #ffffff;
  margin: 0;
  line-height: 1.2;
}
</style>

