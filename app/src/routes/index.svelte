<script context="module">
	import { sources, refresh as sourceRefresh } from '$lib/stores/sources';
	import '../app.css';

	let selectedSource = '';

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

	/** @type {import('./__types/').Load} */
	export async function load({ params, fetch, session, stuff }) {
		await sourceRefresh();
		get(sources);
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
	import { get } from 'svelte/store';
	let address = '';
	const check = async () => {
		await contractRefresh(address, selectedSource);
	};

	const reset = async () => {
		console.log('reset');
		address = '';
		await contractReset();
	};
</script>

{#if $sources.lastError}
	{$sources.lastError}
{:else}
	<h2>Verify contract</h2>
	<select bind:value={selectedSource}>
		{#each $sources.sources as source}
			<option value={source}>
				{source}
			</option>
		{/each}
	</select>

	<input type="text" bind:value={address} />
	<input type="button" value="reset" on:click={reset} />
	<input type="button" value="check" on:click={check} />
	<p>
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
	</p>
{/if}

<footer>built with ❤ by <a href="https://onlydust.xyz">onlydust.xyz</a></footer>
