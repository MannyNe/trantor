<script lang="ts">
	import type { PageData } from './$types';
	import type { ChartData, ChartType } from 'chart.js/auto';

	import { page } from '$app/stores';
	import Chart from '$lib/components/Chart.svelte';

	type CustomChartData<T extends ChartType> = ChartData<T, number[], string>;

	export let data: PageData;
	const trackings = data.trackings;

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
		if (!trackings.session_count_by_weekday.find((s) => s.weekday === i)) {
			trackings.session_count_by_weekday.push({ weekday: i, count: 0 });
		}
		if (!trackings.visitor_count_by_weekday.find((s) => s.weekday === i)) {
			trackings.visitor_count_by_weekday.push({ weekday: i, count: 0 });
		}
	}
	trackings.session_count_by_weekday.sort((a, b) => a.weekday - b.weekday);
	trackings.visitor_count_by_weekday.sort((a, b) => a.weekday - b.weekday);

	const sessionsAndVisitorsChartData: CustomChartData<'bar'> = {
		labels: trackings.session_count_by_weekday.map((s) => weekdayToString[s.weekday]),
		datasets: [
			{
				label: 'Sessions Per Day',
				data: trackings.session_count_by_weekday.map((s) => s.count)
			},
			{
				label: 'Visitors Per Day',
				data: trackings.visitor_count_by_weekday.map((s) => s.count)
			}
		]
	};

	for (let i = 0; i < 24; i++) {
		if (!trackings.session_count_by_hour.find((s) => s.hour === i)) {
			trackings.session_count_by_hour.push({ hour: i, count: 0 });
		}
		if (!trackings.visitor_count_by_hour.find((s) => s.hour === i)) {
			trackings.visitor_count_by_hour.push({ hour: i, count: 0 });
		}
	}
	trackings.session_count_by_hour.sort((a, b) => a.hour - b.hour);
	trackings.visitor_count_by_hour.sort((a, b) => a.hour - b.hour);

	const sessionsAndVisitorsByHourChartData: CustomChartData<'radar'> = {
		labels: trackings.session_count_by_hour.map((s) => s.hour.toString()),
		datasets: [
			{
				label: 'Sessions Per Hour',
				data: trackings.session_count_by_hour.map((s) => s.count)
			},
			{
				label: 'Visitors Per Hour',
				data: trackings.visitor_count_by_hour.map((s) => s.count)
			}
		]
	};

	const visitorsCountByBrowser: CustomChartData<'doughnut'> = {
		labels: trackings.visitor_count_by_browser.map((v) => v.browser),
		datasets: [
			{
				data: trackings.visitor_count_by_browser.map((v) => v.count)
			}
		]
	};
	const visitorsCountByOs: CustomChartData<'doughnut'> = {
		labels: trackings.visitor_count_by_os.map((v) => v.os),
		datasets: [
			{
				data: trackings.visitor_count_by_os.map((v) => v.count)
			}
		]
	};
	const visitorsCountByDevice: CustomChartData<'doughnut'> = {
		labels: trackings.visitor_count_by_device.map((v) => v.device),
		datasets: [
			{
				data: trackings.visitor_count_by_device.map((v) => v.count)
			}
		]
	};

	function copyTrackingId() {
		navigator.clipboard.writeText($page.params.id);
	}
</script>

<svelte:head>
	<title>Tracking - {trackings.name}</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="app">
	<h1>Tracking data for <span>{trackings.name}</span></h1>

	<div class="tracking-id">
		<div>
			<p>Tracking ID</p>
			<h3>{$page.params.id}</h3>
		</div>
		<button on:click={copyTrackingId}>Copy</button>
	</div>

	<section class="stats">
		<div>
			<Chart
				title="Sessions and Visitors Per Day"
				type="bar"
				data={sessionsAndVisitorsChartData}
				options={{ scales: { y: { beginAtZero: true } } }}
			/>
		</div>
		<div>
			<Chart
				title="Sessions and Visitors Per Hour"
				type="radar"
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
</style>
