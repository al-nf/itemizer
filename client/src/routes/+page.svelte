<script lang="ts">
  let championName = "";
  let championData: any = null;
  let abilities: any[] = [];
  let error: string | null = null;

  async function submitChampion() {
    // Set the champion
    const postResponse = await fetch(`http://localhost:8080/setchampion/${championName}`, { 
      method: "POST" 
    });

    if (!postResponse.ok) {
      error = "Failed to set champion data";
      championData = null;
      abilities = [];
      return;
    }

    // Fetch champion details
    try {
      const nameResponse = await fetch(`http://localhost:8080/champion/${championName}/name`);
      const titleResponse = await fetch(`http://localhost:8080/champion/${championName}/title`);
      const iconResponse = await fetch(`http://localhost:8080/champion/${championName}/icon`);
      const abilitiesResponse = await fetch(`http://localhost:8080/champion/${championName}/abilities`);

      if (!nameResponse.ok || !titleResponse.ok || !iconResponse.ok || !abilitiesResponse.ok) {
        throw new Error("Failed to fetch champion data");
      }

      // Parse the data
      const nameData = await nameResponse.json();
      const title = await titleResponse.json();
      const iconData = await iconResponse.json();  // champion icon will come here
      const abilitiesData = await abilitiesResponse.json();

      // Now, extract the abilities from the "P", "Q", "W", "E", "R" keys
      abilities = [
        abilitiesData.P?.[0],  // P Ability
        abilitiesData.Q?.[0],  // Q Ability
        abilitiesData.W?.[0],  // W Ability
        abilitiesData.E?.[0],  // E Ability
        abilitiesData.R?.[0]   // R Ability
      ].filter(Boolean);  // Filters out any undefined or null values

      // Set the champion data
      championData = {
        name: nameData,
        title: title,
        icon: iconData, 
      };

      error = null;
    } catch (err) {
      error = "Champion not found or error occurred";
      championData = null;
      abilities = [];
      console.error(err);  // Log the error for debugging
    }
  }
</script>

<!-- Input form -->
<input type="text" bind:value={championName} placeholder="Enter Champion Name" />
<button on:click={submitChampion}>Get Champion Data</button>

<!-- Error handling -->
{#if error}
  <p>{error}</p>
{:else if championData}
  <!-- Champion Info -->
  <div class="champion-box">
    <img src={championData.icon} alt="{championData.name} Icon" class="champion-icon" />
    <div class="champion-info">
      <h3>{championData.name}</h3>
      <h4>{championData.title}</h4>
    </div>
  </div>

  <!-- Abilities Section -->
  <div class="abilities">
    {#each abilities as ability, index}
      <div class="ability-box">
        <div class="ability-header">
          <!-- Display the ability icon next to the ability name -->
          {#if ability?.icon}
            <img src={ability.icon.replace(/"/g, '')} alt="Ability Icon" class="ability-icon-img" />
          {/if}
          <h5><strong>{index === 0 ? `Passive: ${ability?.name}` : `${['Q', 'W', 'E', 'R'][index - 1]}: ${ability?.name}`}</strong></h5>
        </div>

        <!-- Effects Section: Display the descriptions from the effects array -->
        <div class="ability-effects">
          {#each ability?.effects as effect, effectIndex}
            <p>{effect.description}</p>
          {/each}
        </div>

        <ul>
          <li><strong>Cooldown:</strong> {ability?.cooldown ? ability.cooldown.modifiers[0]?.values.join(" / ") : "N/A"}</li>

          <!-- Damage Section -->
          <li><strong>Damage:</strong>
              {#if ability?.effects}
                {#each ability.effects as effect}
                  {#if effect.leveling}
                    <span>
                      {#each effect.leveling as level, levelIndex}
                        {#if level.modifiers && level.modifiers[0]?.values}
                          {#each level.modifiers[0]?.values as value, idx}
                            {value}{level.modifiers[0]?.units[idx]}
                            {#if idx < level.modifiers[0]?.values.length - 1}{"/ "}{/if}
                          {/each}
                        {/if}
                        {#if levelIndex < effect.leveling.length - 1}{"  ||  "}{/if} <!-- Correctly add " || " with space -->
                      {/each}
                    </span>
                  {/if}
                {/each}
              {/if}
            </li>

          <li><strong>Cost:</strong> {ability?.cost ? ability.cost.modifiers[0]?.values.join(" / ") : "N/A"}</li>
        </ul>
      </div>
    {/each}
  </div>
{/if}

<!-- Styling -->
<style>
  .champion-box {
    display: flex;
    align-items: center;
    background-color: #f3f3f3;
    padding: 10px;
    border-radius: 8px;
    margin-top: 10px;
    max-width: 400px;
  }

  .champion-icon {
    width: 50px;
    height: 50px;
    margin-right: 15px;
    border-radius: 4px;
  }

  .champion-info h3 {
    margin: 0;
    font-size: 1.2em;
  }

  .champion-info h4 {
    margin: 0;
    font-size: 1em;
    color: #555;
  }

  .abilities {
    margin-top: 20px;
  }

  .ability-box {
    margin-bottom: 15px;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 8px;
    background-color: #fff;
  }

  .ability-header {
    display: flex;
    align-items: center;
    margin-bottom: 5px;
  }

  .ability-icon-img {
    width: 30px;
    height: 30px;
    margin-right: 10px;
    border-radius: 50%; /* This makes the image rounded */
  }

  .ability-box h5 {
    margin: 0;
  }

  .ability-box ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .ability-box ul li {
    margin: 5px 0;
  }

  .ability-effects p {
    margin: 5px 0;
    font-size: 0.9em;
    color: #666;
  }
</style>

