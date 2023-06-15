<script lang="ts">
	export let origin: string;
	export let trackingId: string;
</script>

<div class="no-data">
	<img src="/illustrations/no-data.svg" alt="No data" />
	<header>
		<h1>No data yet</h1>
		<p>Include the following script in your website to start tracking</p>
	</header>
	<pre>&lt;script async defer
	src="{origin}/launch-control.js"
	data-id="trantor"
	data-tracking-id="{trackingId}"
&gt;&lt;/script&gt;</pre>
</div>

<style>
	.no-data {
		padding: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		gap: 1rem;
		width: 100%;
		height: 100%;
		font-family: monospace;
	}

	.no-data img {
		width: 100%;
		max-width: 300px;
	}

	.no-data h1 {
		font-size: 2rem;
		font-family: 'Press Start 2P', cursive;
		text-align: center;
	}

	.no-data p {
		font-size: 1.2rem;
		text-align: center;
	}

	.no-data pre {
		padding: 1rem;
		font-size: 1rem;
		background-color: #fff;
		border: 1px solid black;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);

		width: 100%;
		max-width: 600px;
		overflow-x: scroll;
	}
</style>
