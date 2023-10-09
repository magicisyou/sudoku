<script lang="ts">
	import Cell from './atoms/Cell.svelte';
	import VSpace from './atoms/VSpace.svelte';
	import HSpace from './atoms/HSpace.svelte';
	import FreeCell from './atoms/FreeCell.svelte';
	import Button from './atoms/Button.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Store } from 'tauri-plugin-store-api';

	const store = new Store('.settings.dat');

	$: game = { values: [[]] };
	$: unlocked = 0;

	let current_game = 0;

	async function loadStore() {
		let val = await store.get('unlocked');
		if (val) {
			unlocked = val.value;
		}
	}

	async function saveToStore() {
		await store.set('unlocked', { value: unlocked });
		store.save();
	}

	async function check() {
		let result = await invoke('evaluate');
		if (result) {
			document.querySelector('.game').style.border = 'double gold 6px';
			if (current_game == unlocked) {
				unlocked += 1;
				saveToStore();
			}
		} else {
			document.querySelector('.game').style.border = 'double transparent 6px';
		}
	}

	async function inputSync() {
		await invoke('input_sync', {
			index: [Number(this.id[0]), Number(this.id[2])],
			value: Number(this.value)
		});
		check();
	}

	async function newGame() {
		game = await invoke('new_game', { index: 0 });
	}

	async function changeGame() {
		let gameIndex = Number(this.innerText[0]) - 1;
		game = await invoke('new_game', { index: gameIndex });
		document.querySelector('.game').style.border = 'double transparent 6px';
		current_game = gameIndex;
	}
	newGame();
	loadStore();
</script>

<div class="main">
	<div class="flex-column">
		{#each [0, 1, 2, 3, 4, 5, 6, 7] as i}
			{#if i <= unlocked}
				<Button index={i + 1} OnClick={changeGame} lock={false} />
			{:else}
				<Button index={i + 1} />
			{/if}
		{/each}
	</div>
	<div class="game">
		{#each game.values as rows, i}
			{#each rows as val, j}
				{#if val == 0}
					{#key game}
						<FreeCell id={[i, j]} OnInput={inputSync} />
					{/key}
				{:else}
					<Cell value={val} />
				{/if}
				{#if j == 2 || j == 5}
					<VSpace />
				{/if}
				{#if j == 8 && (i == 2 || i == 5)}
					<HSpace />
				{/if}
			{/each}
		{/each}
	</div>
</div>

<style scoped>
	.game {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-around;
		flex-flow: wrap;
		gap: 1px;
		width: 462px;
		border: double transparent 6px;
		padding: 4px;
	}

	.main {
		display: flex;
		flex-direction: row;
		align-items: flex-start;
		justify-content: center;
		gap: 100px;
	}

	.flex-column {
		padding-top: 6px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-around;
		height: 462px;
	}
</style>
