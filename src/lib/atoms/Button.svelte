<script lang="ts">
	export let index = 0;
	export let OnClick = () => {};
	export let lock = true;
</script>

<button on:click={OnClick}>
	{index}
	{#if lock == true}
		<img src="/lock.png" alt="lock" />
	{/if}
</button>

<style scoped>
	button {
		width: 120px;
		height: 50px;
		border-radius: 2px;
		background: #0007;
		color: #d78137;
		border: solid #d78137 1px;
	}

	button:hover {
		background-color: #0003;
	}

	img {
		width: 20px;
		height: 20px;
	}
</style>
