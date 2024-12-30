<script lang="ts">
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let champions: string[] = [];
    let filteredChampions: string[] = [];
    let championName = "";
    let error: string | null = null;
    let showDropdown = false;

    onMount(async () => {
        try {
            const response = await fetch('http://localhost:8080/champion');
            if (!response.ok) throw new Error('Failed to fetch champions');
            const data = await response.json();
            // Get the champion names from the first keys of the object
            champions = Object.keys(data);
            filterChampions();
        } catch (err) {
            console.error(err);
            error = 'Failed to fetch champions';
        }
    });

    function filterChampions() {
        if (!championName.trim()) {
            filteredChampions = champions;
        } else {
            filteredChampions = champions.filter(champion =>
                champion.toLowerCase().includes(championName.toLowerCase())
            );
        }
    }

    function handleInput() {
        filterChampions();
        showDropdown = true;
    }

    function selectChampion(champion: string) {
        championName = champion;
        localStorage.setItem('currentChampion', champion);
        showDropdown = false;
    }

    async function submitChampion() {
        if (!championName) return;
        
        try {
            const response = await fetch(`http://localhost:8080/setchampion/${championName}`, { 
                method: "POST" 
            });

            if (!response.ok) {
                throw new Error("Failed to set champion");
            }

            // If successful, redirect to calculator page
            goto('/calc');
        } catch (err) {
            error = "Failed to set champion";
            console.error(err);
        }
    }
</script>

<div class="container">
    <h1>Champion Select</h1>
    
    <div class="select-container">
        <div class="input-wrapper">
            <input 
                type="text"
                bind:value={championName}
                on:input={handleInput}
                on:focus={() => showDropdown = true}
                placeholder="Type champion name..."
                class="champion-input"
            />
            
            {#if showDropdown && filteredChampions.length > 0}
                <div class="dropdown">
                    {#each filteredChampions as champion}
                        <button 
                            class="dropdown-item"
                            on:click={() => selectChampion(champion)}
                        >
                            {champion}
                        </button>
                    {/each}
                </div>
            {/if}
        </div>

        <button 
            on:click={submitChampion}
            disabled={!championName}
            class="submit-button"
        >
            Continue to Calculator
        </button>
    </div>

    {#if error}
        <p class="error">{error}</p>
    {/if}
</div>

<style>
    .container {
        max-width: 600px;
        margin: 2rem auto;
        padding: 1rem;
    }

    .select-container {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        margin: 2rem 0;
    }

    .input-wrapper {
        position: relative;
    }

    .champion-input {
        width: 100%;
        padding: 0.75rem;
        font-size: 1.1rem;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .dropdown {
        position: absolute;
        top: 100%;
        left: 0;
        right: 0;
        max-height: 300px;
        overflow-y: auto;
        background: white;
        border: 1px solid #ccc;
        border-top: none;
        border-radius: 0 0 4px 4px;
        z-index: 10;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    .dropdown-item {
        width: 100%;
        padding: 0.75rem;
        text-align: left;
        border: none;
        background: none;
        cursor: pointer;
    }

    .dropdown-item:hover {
        background: #f0f0f0;
    }

    .submit-button {
        padding: 0.75rem 1.5rem;
        font-size: 1.1rem;
        background-color: #4CAF50;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .submit-button:disabled {
        background-color: #cccccc;
        cursor: not-allowed;
    }

    .error {
        color: red;
        margin-top: 1rem;
    }
</style>
