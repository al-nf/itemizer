import fs from 'fs';
import fetch from 'node-fetch';

const URL = 'http://localhost:8080/champion';
const OUTPUT_FILE = '../public/champions.json';

async function fetchChampions() {
  try {
    const response = await fetch(URL);
    if (!response.ok) {
      throw new Error('Failed to fetch champions data');
    }

    const data = await response.json();

    const championsData = {};
    for (const key in data) {
      championsData[key] = {
        name: data[key].name,
        icon: data[key].icon,
        title: data[key].title,
      };
    }

    fs.writeFileSync(OUTPUT_FILE, JSON.stringify(championsData, null, 2));
    console.log(`Champions data has been saved to ${OUTPUT_FILE}`);
  } catch (error) {
    console.error('Error fetching or saving champions:', error);
  }
}

fetchChampions();

