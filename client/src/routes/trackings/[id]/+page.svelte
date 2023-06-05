<script lang="ts">
	import type { PageData } from './$types';
	import type { ChartData, ChartType } from 'chart.js/auto';

	import { page } from '$app/stores';
	import { getAuthToken } from '$lib/auth';
	import { invalidateAll } from '$app/navigation';
	import Chart from '$lib/components/Chart.svelte';

	type CustomChartData<T extends ChartType> = ChartData<T, number[], string>;

	export let data: PageData;

	const weekdayToString: Record<number, string> = {
		0: 'Sun',
		1: 'Mon',
		2: 'Tue',
		3: 'Wed',
		4: 'Thu',
		5: 'Fri',
		6: 'Sat'
	};

	for (let i = 0; i < 7; i++) {
		if (!data.tracking.session_count_by_weekday.find((s) => s.weekday === i)) {
			data.tracking.session_count_by_weekday.push({ weekday: i, count: 0 });
		}
		if (!data.tracking.visitor_count_by_weekday.find((s) => s.weekday === i)) {
			data.tracking.visitor_count_by_weekday.push({ weekday: i, count: 0 });
		}
	}
	data.tracking.session_count_by_weekday.sort((a, b) => a.weekday - b.weekday);
	data.tracking.visitor_count_by_weekday.sort((a, b) => a.weekday - b.weekday);

	const sessionsAndVisitorsChartData: CustomChartData<'bar'> = {
		labels: data.tracking.session_count_by_weekday.map((s) => weekdayToString[s.weekday]),
		datasets: [
			{
				label: 'Sessions Per Day',
				data: data.tracking.session_count_by_weekday.map((s) => s.count)
			},
			{
				label: 'Visitors Per Day',
				data: data.tracking.visitor_count_by_weekday.map((s) => s.count)
			}
		]
	};

	for (let i = 0; i < 24; i++) {
		if (!data.tracking.session_count_by_hour.find((s) => s.hour === i)) {
			data.tracking.session_count_by_hour.push({ hour: i, count: 0 });
		}
		if (!data.tracking.visitor_count_by_hour.find((s) => s.hour === i)) {
			data.tracking.visitor_count_by_hour.push({ hour: i, count: 0 });
		}
	}
	data.tracking.session_count_by_hour.sort((a, b) => a.hour - b.hour);
	data.tracking.visitor_count_by_hour.sort((a, b) => a.hour - b.hour);

	const sessionsAndVisitorsByHourChartData: CustomChartData<'radar'> = {
		labels: data.tracking.session_count_by_hour.map((s) => s.hour.toString()),
		datasets: [
			{
				label: 'Sessions Per Hour',
				data: data.tracking.session_count_by_hour.map((s) => s.count)
			},
			{
				label: 'Visitors Per Hour',
				data: data.tracking.visitor_count_by_hour.map((s) => s.count)
			}
		]
	};

	const visitorsCountByBrowser: CustomChartData<'doughnut'> = {
		labels: data.tracking.visitor_count_by_browser.map((v) => v.browser),
		datasets: [
			{
				data: data.tracking.visitor_count_by_browser.map((v) => v.count)
			}
		]
	};
	const visitorsCountByOs: CustomChartData<'doughnut'> = {
		labels: data.tracking.visitor_count_by_os.map((v) => v.os),
		datasets: [
			{
				data: data.tracking.visitor_count_by_os.map((v) => v.count)
			}
		]
	};
	const visitorsCountByDevice: CustomChartData<'doughnut'> = {
		labels: data.tracking.visitor_count_by_device.map((v) => v.device),
		datasets: [
			{
				data: data.tracking.visitor_count_by_device.map((v) => v.count)
			}
		]
	};

	function copyTrackingId() {
		navigator.clipboard.writeText($page.params.id);
	}

	async function handleAddSource(event: Event) {
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);
		const name = formData.get('name') as string;

		const authToken = getAuthToken();

		const res = await fetch(`/admin/trackings/${$page.params.id}/sources`, {
			method: 'POST',
			headers: {
				Authorization: `Basic ${authToken}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ name })
		});

		if (res.status !== 201) {
			alert('Something went wrong');
		}

		form.reset();
		await invalidateAll();
	}

	async function deleteSource(name: string) {
		const confirmed = confirm(`Are you sure you want to delete "${name}"?`);
		if (!confirmed) return;

		const authToken = getAuthToken();

		const res = await fetch(`/admin/trackings/${$page.params.id}/sources/${name}`, {
			method: 'DELETE',
			headers: {
				Authorization: `Basic ${authToken}`
			}
		});

		if (res.status !== 204) {
			alert('Something went wrong');
		}

		await invalidateAll();
	}
</script>

<svelte:head>
	<title>{data.tracking.name} | Trantor</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="app">
	<h1>Tracking data for <span>{data.tracking.name}</span></h1>

	<div class="tracking-id">
		<div>
			<p>Tracking ID</p>
			<h3>{$page.params.id}</h3>
		</div>
		<button on:click={copyTrackingId}>Copy</button>
	</div>

	<section class="stats">
		<div>
			<Chart title="Sessions and Visitors Per Day" type="bar" data={sessionsAndVisitorsChartData} />
		</div>
		<div>
			<Chart
				type="radar"
				title="Sessions and Visitors Per Hour"
				data={sessionsAndVisitorsByHourChartData}
			/>
		</div>
		<div>
			<Chart title="Visitors By Browser" type="doughnut" data={visitorsCountByBrowser} />
		</div>
		<div>
			<Chart title="Visitors By OS" type="doughnut" data={visitorsCountByOs} />
		</div>
		<div>
			<Chart title="Visitors By Device" type="doughnut" data={visitorsCountByDevice} />
		</div>
	</section>

	<div class="two-columns">
		<section class="table-container">
			<h1>Sources</h1>

			<form class="add-source" on:submit|preventDefault={handleAddSource}>
				<input required name="name" placeholder="Name" type="text" autocomplete="off" />
				<button type="submit">Add Source</button>
			</form>
			<table>
				<thead>
					<th>Source Name</th>
					<th>Session Count</th>
					<th>Visitor Count</th>
					<th style="border-right: 1px solid #000;" />
				</thead>
				{#each data.sources as source}
					<tr>
						<td>{source.name}</td>
						<td>{source.session_count}</td>
						<td>{source.visitor_count}</td>
						<td>
							{#if source.name !== 'direct'}
								<button on:click={() => deleteSource(source.name)}>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										fill="none"
										viewBox="0 0 24 24"
										stroke-width="1.5"
										stroke="currentColor"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
										/>
									</svg>
								</button>
							{/if}
						</td>
					</tr>
				{/each}
			</table>
		</section>

		<section class="table-container">
			<h1>Paths</h1>

			<table>
				<thead>
					<th>Path</th>
					<th style="border-right: 1px solid #000;">Session Count</th>
				</thead>
				{#each data.paths as path}
					<tr>
						<td>{path.pathname}</td>
						<td>{path.count}</td>
					</tr>
				{/each}
			</table>
		</section>
	</div>

	<div class="two-columns">
		<section class="table-container">
			<h1>Page Titles</h1>

			<table>
				<thead>
					<th>Title</th>
					<th style="border-right: 1px solid #000;">Session Count</th>
				</thead>
				{#each data.titles as title}
					<tr>
						<td>{title.title}</td>
						<td>{title.count}</td>
					</tr>
				{/each}
			</table>
		</section>
	</div>
</div>

<style>
	.app {
		width: 80vw;
		margin: 1rem auto;
	}

	.app h1 {
		font-size: 2rem;
		margin-bottom: 2rem;
		font-family: 'Press Start 2P', cursive;
		line-height: 3rem;
	}

	.app h1 span {
		padding: 0.5rem 1rem;
		background-color: black;
		color: white;
	}

	.tracking-id {
		background-color: black;
		color: white;

		display: flex;
		align-items: center;
		justify-content: space-between;

		width: fit-content;

		margin-bottom: 2rem;
		border: 1px solid #000;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);
	}

	.tracking-id div {
		padding: 0.5rem 1rem;
	}

	.tracking-id h3 {
		font-size: 1.5rem;
		font-family: 'Press Start 2P', cursive;
	}

	.tracking-id p {
		font-family: monospace;
		margin-bottom: 10px;
	}

	.tracking-id button {
		height: 100%;
		font-family: monospace;
		padding: 1.4rem 1rem;
		background-color: blueviolet;
		border: none;
		color: white;
		text-transform: uppercase;
		border-left: 1px solid #fff;

		font-size: 1rem;
		font-weight: bold;
		cursor: pointer;
	}

	.tracking-id button:hover,
	.tracking-id button:focus {
		background-color: #fff;
		color: blueviolet;
	}

	.tracking-id button:active {
		transform: scale(0.9);
	}

	.stats {
		padding: 1rem;
		display: flex;
		overflow-x: scroll;
		border: 2px solid #000;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);
	}

	/* style stats scroll bar */
	.stats::-webkit-scrollbar {
		height: 5px;
	}
	.stats::-webkit-scrollbar-track {
		background: transparent;
	}
	.stats::-webkit-scrollbar-thumb {
		background: #000;
	}

	.stats > div {
		width: 400px;
		height: 400px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	@media (max-width: 810px) {
		.app {
			width: 90vw;
		}

		.app h1 {
			font-size: 1.5rem;
		}

		.tracking-id h3 {
			font-size: 0.6rem;
		}
	}

	@media (max-width: 500px) {
		.app {
			width: 100vw;
			padding: 1rem;
		}

		.stats {
			padding: 0.5rem;
		}
	}

	.two-columns {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 2rem;
		margin-top: 2rem;
	}

	@media (max-width: 1100px) {
		.two-columns {
			grid-template-columns: 1fr;
		}
	}

	.table-container {
		padding: 1rem;
		border: 2px solid #000;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);
	}

	.table-container h1 {
		font-size: 1.5rem;
		font-family: 'Press Start 2P', cursive;
		margin-bottom: 1rem;
	}

	table {
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		border: 1px solid #000;
		padding: 0.5rem;
	}

	th {
		text-align: left;
		font-size: 0.8rem;
		font-family: monospace;
		border-right: 1px solid #fff;
	}

	td,
	th {
		font-family: monospace;
	}

	thead {
		color: #fff;
		background-color: #000;
	}

	th:nth-child(2),
	th:nth-child(3),
	td:nth-child(2),
	td:nth-child(3) {
		text-align: right;
		font-weight: bolder;
	}

	th:nth-child(4),
	td:nth-child(4) {
		padding: 0;
	}

	table button {
		width: 100%;
		padding: 0.5rem;
		background-color: red;
		color: white;
		border: none;
		cursor: pointer;
	}

	table button svg {
		width: 1rem;
		height: 1rem;
	}

	table button:hover,
	table button:focus {
		background-color: #ff0000aa;
	}

	table button:active {
		transform: scale(0.9);
	}

	.add-source {
		display: grid;
		grid-template-columns: 70% 30%;
		margin-bottom: 1rem;
	}

	.add-source input {
		padding: 0.5rem;
		font-family: monospace;
		border: 1px solid #000;
		background-color: transparent;
	}

	.add-source button {
		padding: 0.5rem;
		font-family: 'Press Start 2P', cursive;
		background-color: black;
		color: white;
		border: none;
		cursor: pointer;
	}

	.add-source button:hover,
	.add-source button:focus {
		background-color: #000000aa;
	}

	.add-source button:active {
		transform: scale(0.9);
	}
</style>
