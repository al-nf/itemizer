<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let championData: any = null;
    let abilities: any[] = [];
    let playerStats: any = null;
    let error: string | null = null;
    let loading: boolean = true;

    onMount(async () => {
        const selectedChampion = localStorage.getItem('currentChampion');
        if (!selectedChampion) {
            console.error('No champion data received');
            goto('/');
            return;
        }

        try {
            const nameResponse = await fetch(`http://localhost:8080/champion/${selectedChampion}/name`);
            const titleResponse = await fetch(`http://localhost:8080/champion/${selectedChampion}/title`);
            const iconResponse = await fetch(`http://localhost:8080/champion/${selectedChampion}/icon`);
            const abilitiesResponse = await fetch(`http://localhost:8080/champion/${selectedChampion}/abilities`);

            if (!nameResponse.ok || !titleResponse.ok || !iconResponse.ok || !abilitiesResponse.ok) {
                throw new Error("Failed to fetch champion data");
            }

            const nameData = await nameResponse.json();
            const title = await titleResponse.json();
            const iconData = await iconResponse.json();
            const abilitiesData = await abilitiesResponse.json();

            abilities = [
                abilitiesData.P?.[0],
                abilitiesData.Q?.[0],
                abilitiesData.W?.[0],
                abilitiesData.E?.[0],
                abilitiesData.R?.[0]
            ].filter(Boolean);

            championData = {
                name: nameData,
                title: title,
                icon: iconData,
            };
        } catch (err) {
            console.error('Error loading champion:', err);
            error = "Failed to load champion data";
        }

        try {
            const response = await fetch('http://localhost:8080/player');
            if (!response.ok) {
                throw new Error("Failed to fetch player stats");
            }
            playerStats = await response.json();
        } catch (err) {
            console.error('Error loading player stats:', err);
            error = "Failed to load player stats";
        } finally {
            loading = false;
        }
    });

    function handleBack() {
        goto('/');
    }
</script>

<div class="calculator-page">
    <button class="back-button" on:click={handleBack}>
        ← Back to Champion Select
    </button>

    {#if loading}
        <p>Loading data, please wait...</p>
    {:else if error}
        <p class="error">{error}</p>
    {:else if championData}
        <div class="champion-header">
            <img src={championData.icon} alt="{championData.name} Icon" class="champion-icon" />
            <div class="champion-info">
                <h1>{championData.name}</h1>
                <h2>{championData.title}</h2>
            </div>
        </div>

        <div class="abilities-grid">
            {#each abilities as ability, index}
                <div class="ability-card">
                    <div class="ability-header">
                        {#if ability?.icon}
                            <img src={ability.icon} alt="Ability Icon" class="ability-icon" />
                        {/if}
                        <h3>{index === 0 ? 'Passive' : ['Q', 'W', 'E', 'R'][index - 1]}: {ability?.name}</h3>
                    </div>
                    
                    <div class="ability-details">
                        {#each ability?.effects || [] as effect}
                            <p>{effect.description}</p>
                        {/each}
                        
                        {#if ability?.cooldown}
                            <p class="ability-stat">Cooldown: {ability.cooldown.modifiers[0]?.values.join(" / ")}</p>
                        {/if}
                        
                        {#if ability?.cost}
                            <p class="ability-stat">Cost: {ability.cost.modifiers[0]?.values.join(" / ")}</p>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>

        <div class="player-stats">
            <h2>Player Stats</h2>
            <ul>
                {#each Object.entries(playerStats.stats) as [key, stat]}
                    {#if key !== 'gold_per_10'}
                        <li>
                            {#if key === 'armor_penetration'}
                                <strong>{key}: </strong> {stat.percent}%
                            {:else if key === 'magic_penetration'}
                                <strong>{key}: </strong> {stat.flat}, {stat.percent}%
                            {:else}
                                <strong>{key}: </strong> {stat.flat}
                            {/if}
                        </li>
                    {/if}
                {/each}
            </ul>
        </div>

        <div class="calculator-section">
            <h2>Damage Calculator</h2>
        </div>
    {:else}
        <p>Loading champion data...</p>
    {/if}
</div>

<style>
    .calculator-page {
        padding: 2rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .back-button {
        margin-bottom: 2rem;
        padding: 0.5rem 1rem;
        background-color: #f0f0f0;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .champion-header {
        display: flex;
        align-items: center;
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .champion-icon {
        width: 80px;
        height: 80px;
        border-radius: 50%;
    }

    .abilities-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
    }

    .ability-card {
        background: #f5f5f5;
        border-radius: 8px;
        padding: 1rem;
    }

    .ability-header {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .ability-icon {
        width: 40px;
        height: 40px;
        border-radius: 4px;
    }

    .ability-details {
        font-size: 0.9rem;
    }

    .ability-stat {
        color: #666;
        margin-top: 0.5rem;
    }

    .player-stats {
        margin-top: 2rem;
        background: #f9f9f9;
        padding: 1rem;
        border-radius: 8px;
    }

    .error {
        color: red;
        padding: 1rem;
        background: #fff5f5;
        border-radius: 4px;
    }
</style>

