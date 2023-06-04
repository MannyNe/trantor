<script lang="ts">
	import type { PageData } from './$types';
	import type { ChartData, ChartType } from 'chart.js/auto';

	import { page } from '$app/stores';
	import Chart from '$lib/components/Chart.svelte';

	type CustomChartData<T extends ChartType> = ChartData<T, number[], string>;

	export let data: PageData;
	const tracking = data.tracking;
	const sources = data.sources.sort((a, b) => b.session_count - a.session_count);
	const paths = data.paths.sort((a, b) => b.count - a.count);

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
		if (!tracking.session_count_by_weekday.find((s) => s.weekday === i)) {
			tracking.session_count_by_weekday.push({ weekday: i, count: 0 });
		}
		if (!tracking.visitor_count_by_weekday.find((s) => s.weekday === i)) {
			tracking.visitor_count_by_weekday.push({ weekday: i, count: 0 });
		}
	}
	tracking.session_count_by_weekday.sort((a, b) => a.weekday - b.weekday);
	tracking.visitor_count_by_weekday.sort((a, b) => a.weekday - b.weekday);

	const sessionsAndVisitorsChartData: CustomChartData<'bar'> = {
		labels: tracking.session_count_by_weekday.map((s) => weekdayToString[s.weekday]),
		datasets: [
			{
				label: 'Sessions Per Day',
				data: tracking.session_count_by_weekday.map((s) => s.count)
			},
			{
				label: 'Visitors Per Day',
				data: tracking.visitor_count_by_weekday.map((s) => s.count)
			}
		]
	};

	for (let i = 0; i < 24; i++) {
		if (!tracking.session_count_by_hour.find((s) => s.hour === i)) {
			tracking.session_count_by_hour.push({ hour: i, count: 0 });
		}
		if (!tracking.visitor_count_by_hour.find((s) => s.hour === i)) {
			tracking.visitor_count_by_hour.push({ hour: i, count: 0 });
		}
	}
	tracking.session_count_by_hour.sort((a, b) => a.hour - b.hour);
	tracking.visitor_count_by_hour.sort((a, b) => a.hour - b.hour);

	const sessionsAndVisitorsByHourChartData: CustomChartData<'radar'> = {
		labels: tracking.session_count_by_hour.map((s) => s.hour.toString()),
		datasets: [
			{
				label: 'Sessions Per Hour',
				data: tracking.session_count_by_hour.map((s) => s.count)
			},
			{
				label: 'Visitors Per Hour',
				data: tracking.visitor_count_by_hour.map((s) => s.count)
			}
		]
	};

	const visitorsCountByBrowser: CustomChartData<'doughnut'> = {
		labels: tracking.visitor_count_by_browser.map((v) => v.browser),
		datasets: [
			{
				data: tracking.visitor_count_by_browser.map((v) => v.count)
			}
		]
	};
	const visitorsCountByOs: CustomChartData<'doughnut'> = {
		labels: tracking.visitor_count_by_os.map((v) => v.os),
		datasets: [
			{
				data: tracking.visitor_count_by_os.map((v) => v.count)
			}
		]
	};
	const visitorsCountByDevice: CustomChartData<'doughnut'> = {
		labels: tracking.visitor_count_by_device.map((v) => v.device),
		datasets: [
			{
				data: tracking.visitor_count_by_device.map((v) => v.count)
			}
		]
	};

	function copyTrackingId() {
		navigator.clipboard.writeText($page.params.id);
	}
</script>

<svelte:head>
	<title>{tracking.name} | Tracking Data</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="app">
	<h1>Tracking data for <span>{tracking.name}</span></h1>

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

			<table>
				<thead>
					<th>Source Name</th>
					<th>Session Count</th>
					<th>Visitor Count</th>
				</thead>
				{#each sources as source}
					<tr>
						<td>{source.name}</td>
						<td>{source.session_count}</td>
						<td>{source.visitor_count}</td>
					</tr>
				{/each}
			</table>
		</section>

		<section class="table-container">
			<h1>Paths</h1>

			<table>
				<thead>
					<th>Path</th>
					<th>Session Count</th>
				</thead>
				{#each paths as path}
					<tr>
						<td>{path.pathname}</td>
						<td>{path.count}</td>
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
		scale: 0.9;
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
	}

	td,
	th {
		font-family: monospace;
	}

	thead {
		background-color: #000;
		color: #fff;
	}

	th:nth-child(2),
	th:nth-child(3),
	td:nth-child(2),
	td:nth-child(3) {
		text-align: right;
		font-weight: bolder;
	}
</style>
