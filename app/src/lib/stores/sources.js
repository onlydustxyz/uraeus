import { writable } from 'svelte/store';

export const refresh = async () => {
	const endpoint = `/api/sources`;
	try {
		sources.update((v) => ({
			...v,
			reload: true
		}));
		const response = await fetch(endpoint, {
			headers: {
				'Content-Type': 'application/json'
			}
		});
		let data = await response.json();
		sources.update(() => ({
			...data,
			reload: false
		}));
	} catch (error) {
		console.log(error);
		sources.update((v) => ({
			...v,
			lastError: `could not fetch ${endpoint}`,
			reload: false
		}));
	}
};

export const sources = writable({
	reload: false,
	lastError: ``,
	sources: []
});
