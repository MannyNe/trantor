<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { getAuthToken } from '$lib/auth';

	async function deleteTracking() {
		const confirmed = confirm('Are you sure you want to delete this tracking?');
		if (!confirmed) return;

		const authToken = getAuthToken();

		const res = await fetch(`/admin/trackings/${$page.params.id}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Basic ${authToken}`
			}
		});

		if (res.status !== 204) {
			alert('Something went wrong');
		}

		await goto('/');
	}
</script>

<h1>Settings Page</h1>
<button on:click={deleteTracking}>DELETE TRACKING</button>

<style>
	h1 {
		font-size: 1.5rem;
		font-family: 'Press Start 2P', cursive;
		margin: 1rem 0;
	}

	button {
		background-color: #ff0000;
		color: #fff;
		border: none;
		padding: 0.5rem 1rem;
		font-family: 'Press Start 2P', cursive;
		cursor: pointer;
	}

	button:hover,
	button:focus {
		background-color: #ff3333;
	}

	button:active {
		background-color: #ff6666;
		transform: scale(0.9);
	}
</style>
