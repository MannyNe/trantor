<script lang="ts">
	import './styles.css';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { isAuthenticated, authState } from '$lib/auth';

	isAuthenticated.subscribe((authenticated) => {
		if (authenticated) {
			goto($page.url.pathname);
		} else {
			goto('/login');
		}
	});
</script>

<section>
	<header>
		<a href="/" class="icon">
			<img src="/icon.svg" alt="icon" />
			<h1>TRANTOR</h1>
		</a>
		{#if $isAuthenticated}
			<button on:click={() => authState.set(null)}>Log out</button>
		{/if}
	</header>

	<main>
		<slot />
	</main>
</section>

<style>
	a {
		text-decoration: none;
		color: inherit;
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	img {
		width: 50px;
	}

	h1 {
		color: #fff;
		font-family: 'Press Start 2P', cursive;
	}

	section {
		display: grid;
		grid-template-rows: 100px 1fr;
		min-height: 100vh;
	}

	.icon {
		padding: 1rem;
		gap: 0.5rem;
		display: flex;
		align-items: center;
		background-color: black;
		height: 100%;
	}

	button {
		padding: 1rem;
		height: 100%;
		border: 1px solid #000;
		font-family: 'Press Start 2P', cursive;
		cursor: pointer;
		background-color: transparent;
		font-size: 1rem;
	}

	button:hover {
		background-color: #000;
		color: #fff;
	}

	@media (max-width: 500px) {
		.icon {
			width: 100%;
		}
	}
</style>
