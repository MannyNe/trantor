<script lang="ts">
	import { page } from '$app/stores';
</script>

<section>
	<img src="/illustrations/error-page.svg" alt="Error page" />
	<div>
		<h1>{$page.status}</h1>
		<h1>{$page.error?.message}</h1>
	</div>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 2rem;
		padding: 2rem;
	}

	section img {
		width: 100%;
		max-width: 450px;
	}

	div h1 {
		font-size: 2rem;
		font-family: 'Press Start 2P', cursive;
		text-align: center;
	}
</style>
