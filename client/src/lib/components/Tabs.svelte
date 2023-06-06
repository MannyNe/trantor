<script lang="ts">
	type Tab = {
		name: string;
		icon: {
			emoji: string;
			label: string;
		};
		path: string;
		active: boolean;
	};

	export let tabs: Tab[];
</script>

<section>
	{#each tabs as tab}
		<a href={tab.path} class={tab.active ? 'active' : ''}>
			<span role="img" aria-label={tab.icon.label}>
				{tab.icon.emoji}
			</span>
			<span>{tab.name}</span>
		</a>
	{/each}
</section>

<style>
	section {
		width: 100%;
		font-size: 1.25rem;
		border-bottom: 2px solid #000;
		display: flex;
	}

	section a {
		text-decoration: none;
		color: black;
		padding: 0.5rem 1rem;
		border-right: 2px solid #000;

		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.25rem;

		font-family: monospace;
	}

	section a:hover,
	section a:focus {
		background-color: #eee;
	}

	section a:active {
		background-color: #ddd;
	}

	.active {
		font-weight: 700;
		color: white;
		background-color: #000;
	}

	.active:hover {
		background-color: #000000aa;
	}
</style>
