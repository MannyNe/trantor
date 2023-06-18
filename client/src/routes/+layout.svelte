<script lang="ts">
	import './styles.css';
	import { authState, initAuth } from '$lib/auth.store';
	import { onMount } from 'svelte';

	onMount(() => {
		initAuth();
	});

</script>

<section>
	<header>
		<a href="/" class="icon">
			<img src="/icon.svg" alt="icon" />
			<h1>TRANTOR</h1>
		</a>
		{#if $authState !== null}
			<button on:click={() => authState.set(null)}>LOG OUT</button>
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

	a:hover,
	a:focus {
		background-color: #000000aa;
	}

	a:active {
		transform: scale(0.9);
	}

	header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	img {
		width: 30px;
		height: 55px;
	}

	h1 {
		color: #fff;
		font-family: 'Press Start 2P', cursive;
	}

	section {
		display: grid;
		grid-template-rows: auto 1fr;
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
		height: 100%;
		padding: 1rem;
		font-size: 1rem;
		font-family: 'Press Start 2P', cursive;
		background-color: #000;
		color: #fff;
		cursor: pointer;
		border: none;
	}

	button:hover,
	button:focus {
		background-color: #000000aa;
	}

	button:active {
		transform: scale(0.9);
	}

	@media (max-width: 500px) {
		.icon {
			width: 100%;
		}
	}
</style>
