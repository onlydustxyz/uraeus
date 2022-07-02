import { writable } from 'svelte/store';

export const contract = writable({
	init: true,
	loading: false,
	lastError: '',
	address: '',
	source: '',
	success: false
});

export const reset = async () => {
	contract.update(() => ({
		init: true,
		loading: false,
		lastError: '',
		address: '',
		source: '',
		success: false
	}));
};

export const refresh = async (
	/** @type {string} */
	address,
	/** @type {string} */
	source
) => {
	const endpoint = `/api/verify`;
	try {
		contract.update((v) => ({
			...v,
			init: false,
			success: false,
			lastError: '',
			loading: true
		}));
		const response = await fetch(endpoint, {
			headers: {
				'Content-Type': 'application/json'
			},
			method: 'POST',
			body: JSON.stringify({
				address,
				source
			})
		});
		let data = await response.json();
		contract.update((v) => ({
			...v,
			...data,
			lastError: '',
			loading: false
		}));
	} catch (error) {
		contract.update((v) => ({
			...v,
			success: false,
			lastError: `error fetching ${endpoint}`,
			loading: false
		}));
	}
};
