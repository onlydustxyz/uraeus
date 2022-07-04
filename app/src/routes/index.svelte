<script context="module">
	import { sources, refresh as sourceRefresh } from '$lib/stores/sources';

	let selectedSource = '';

	// subscribe to the store of contract files and select the `main` file
	// if it exists. If it does not and there is a file, select the first one.
	sources.subscribe((value) => {
		let found = -1;
		value.sources.forEach((value, idx) => {
			if (value === 'main') {
				found = idx;
			}
		});
		if (found > -1) {
			selectedSource = value.sources[found];
		} else {
			selectedSource = value.sources[0];
		}
	});

	// load for the load of sources on the first page load.
	/** @type {import('./__types/').Load} */
	export async function load() {
		await sourceRefresh();
		return {
			status: 200
		};
	}
</script>

<script>
	import {
		contract,
		refresh as contractRefresh,
		reset as contractReset
	} from '$lib/stores/contract';

	let address = '';

	// check runs a contract refresh and check if the contract and source
	// matches. to do this, it relies on the address and the selectedSource
	// that are set in the page with the `bind:value` property.
	const check = async () => {
		await contractRefresh(address, selectedSource);
	};

	// reset cleans the form and contract store.
	const reset = async () => {
		console.log('reset');
		address = '';
		await contractReset();
	};
</script>

<main>
	{#if $sources.lastError}
		{$sources.lastError}
	{:else}
		<h2>Verify contract</h2>
		<label for="source">Source file:</label>
		<select class="source" id="source" bind:value={selectedSource}>
			{#each $sources.sources as source}
				<option value={source}>
					{source}
				</option>
			{/each}
		</select>

		<label for="address">Address</label>
		<input id="address" class="address" type="text" bind:value={address} />
		<div class="actions">
			<input class="reset button" type="button" value="reset" on:click={reset} />
			<input class="check button" type="button" value="check" on:click={check} />
		</div>
		<div class="message">
			{#if !$contract.init}
				{#if $contract.loading}
					Loading...
				{:else if $contract.lastError}
					{$contract.lastError}
				{:else if $contract.success}
					✅ {$contract.source}.cairo matches the address
				{:else}
					❌ {$contract.source}.cairo does not match the address
				{/if}
			{/if}
		</div>
	{/if}
</main>

<style>
	h2 {
		margin: 0px;
		padding-top: 2em;
		padding-bottom: 1em;
		font-size: 2em;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: top;
		padding: 1rem;
		width: 100%;
		max-width: 1024px;
		margin: 0 auto;
		box-sizing: border-box;
	}

	label {
		display: block;
		padding-left: 0.5em;
		margin-top: 1em;
		font-size: 1.2rem;
	}

	.actions {
		padding: 1rem;
		display: flex;
		justify-content: end;
	}

	.source {
		margin: 0.5rem;
		border-radius: 16px;
		border: none;
		padding-left: 0.5em;
		padding-right: 0.5em;
		font-size: 1.5rem;
	}

	.address {
		margin: 0.5rem;
		border-radius: 16px;
		border: none;
		padding-left: 0.5em;
		padding-right: 0.5em;
		font-size: 1.5rem;
	}

	.message {
		align-content: center;
		justify-content: center;
		margin: 0 auto;
		padding-top: 1.5rem;
		border: none;
		font-size: 1.5rem;
	}

	.button {
		background-color: hotpink;
		margin: 0.5rem;
		min-width: 6em;
		border-radius: 16px;
		border: none;
		padding-left: 0.5em;
		padding-right: 0.5em;
		font-size: 1.2em;
		height: 2em;
	}
</style>
