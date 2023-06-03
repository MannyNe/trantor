<script lang="ts">
	import chartjs from 'chart.js/auto';
	import autocolors from 'chartjs-plugin-autocolors';

	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	import { page } from '$app/stores';

	export let data: PageData;
	let sessionsAndVisitorsChart: HTMLCanvasElement;
	let sessionsAndVisitorsChartCtx: CanvasRenderingContext2D;

	let sessionsAndVisitorsByHourChart: HTMLCanvasElement;
	let sessionsAndVisitorsByHourChartCtx: CanvasRenderingContext2D;

	let visitorsCountByBrowserChart: HTMLCanvasElement;
	let visitorsCountByBrowserChartCtx: CanvasRenderingContext2D;

	let visitorsCountByOsChart: HTMLCanvasElement;
	let visitorsCountByOsChartCtx: CanvasRenderingContext2D;

	let visitorsCountByDeviceChart: HTMLCanvasElement;
	let visitorsCountByDeviceChartCtx: CanvasRenderingContext2D;

	const sessionsOnDay = (day: number) => {
		return data.session_count_by_weekday.find((s) => s.weekday === day)?.count || 0;
	};
	const visitorsOnDay = (day: number) => {
		return data.visitor_count_by_weekday.find((s) => s.weekday === day)?.count || 0;
	};

	const sessionsAndVisitorsChartData = {
		labels: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
		datasets: [
			{
				label: 'Sessions Per Day',
				data: [
					sessionsOnDay(1),
					sessionsOnDay(2),
					sessionsOnDay(3),
					sessionsOnDay(4),
					sessionsOnDay(5),
					sessionsOnDay(6),
					sessionsOnDay(0)
				]
			},
			{
				label: 'Visitors Per Day',
				data: [
					visitorsOnDay(1),
					visitorsOnDay(2),
					visitorsOnDay(3),
					visitorsOnDay(4),
					visitorsOnDay(5),
					visitorsOnDay(6),
					visitorsOnDay(0)
				]
			}
		]
	};

	const sessionsOnHour = (hour: number) => {
		return data.session_count_by_hour.find((s) => s.hour === hour)?.count || 0;
	};
	const visitorsOnHour = (hour: number) => {
		return data.visitor_count_by_hour.find((s) => s.hour === hour)?.count || 0;
	};

	const sessionsAndVisitorsByHourChartData = {
		labels: new Array(24).fill(0).map((_, i) => i),
		datasets: [
			{
				label: 'Sessions Per Hour',
				data: new Array(24).fill(null).map((_, i) => sessionsOnHour(i))
			},
			{
				label: 'Visitors Per Hour',
				data: new Array(24).fill(null).map((_, i) => visitorsOnHour(i))
			}
		]
	};

	const visitorsCountByBrowser = {
		labels: data.visitor_count_by_browser.map((v) => v.browser),
		datasets: [
			{
				data: data.visitor_count_by_browser.map((v) => v.count)
			}
		]
	};

	const visitorsCountByOs = {
		labels: data.visitor_count_by_os.map((v) => v.os),
		datasets: [
			{
				data: data.visitor_count_by_os.map((v) => v.count)
			}
		]
	};

	const visitorsCountByDevice = {
		labels: data.visitor_count_by_device.map((v) => v.device),
		datasets: [
			{
				data: data.visitor_count_by_device.map((v) => v.count)
			}
		]
	};

	onMount(async () => {
		sessionsAndVisitorsChartCtx = sessionsAndVisitorsChart.getContext('2d')!;
		new chartjs(sessionsAndVisitorsChartCtx, {
			type: 'bar',
			data: sessionsAndVisitorsChartData,
			options: {
				plugins: {
					legend: {
						labels: {
							font: {
								family: 'monospace'
							}
						}
					},
					title: {
						display: true,
						text: 'Sessions and Visitors Per Day'
					},
					autocolors
				},
				scales: {
					y: {
						beginAtZero: true
					}
				},
				aspectRatio: 1
			}
		});

		sessionsAndVisitorsByHourChartCtx = sessionsAndVisitorsByHourChart.getContext('2d')!;
		new chartjs(sessionsAndVisitorsByHourChartCtx, {
			type: 'radar',
			data: sessionsAndVisitorsByHourChartData,
			options: {
				plugins: {
					legend: {
						labels: {
							font: {
								family: 'monospace'
							}
						}
					},
					title: {
						display: true,
						text: 'Sessions and Visitors Per Hour'
					},
					autocolors
				},
				aspectRatio: 1
			}
		});

		visitorsCountByBrowserChartCtx = visitorsCountByBrowserChart.getContext('2d')!;
		new chartjs(visitorsCountByBrowserChartCtx, {
			type: 'doughnut',
			data: visitorsCountByBrowser,
			options: {
				plugins: {
					legend: {
						labels: {
							font: {
								family: 'monospace'
							}
						}
					},
					title: {
						display: true,
						text: 'Visitors By Browser'
					},
					autocolors
				},
				aspectRatio: 1
			}
		});

		visitorsCountByOsChartCtx = visitorsCountByOsChart.getContext('2d')!;
		new chartjs(visitorsCountByOsChartCtx, {
			type: 'doughnut',
			data: visitorsCountByOs,
			options: {
				plugins: {
					legend: {
						labels: {
							font: {
								family: 'monospace'
							}
						}
					},
					title: {
						display: true,
						text: 'Visitors By OS'
					},
					autocolors
				},
				aspectRatio: 1
			}
		});

		visitorsCountByDeviceChartCtx = visitorsCountByDeviceChart.getContext('2d')!;
		new chartjs(visitorsCountByDeviceChartCtx, {
			type: 'doughnut',
			data: visitorsCountByDevice,
			options: {
				plugins: {
					legend: {
						labels: {
							font: {
								family: 'monospace'
							}
						}
					},
					title: {
						display: true,
						text: 'Visitors By Device'
					},
					autocolors
				},
				aspectRatio: 1
			}
		});
	});

	function copyTrackingId() {
		navigator.clipboard.writeText($page.params.id);
	}
</script>

<svelte:head>
	<title>Tracking - {data.name}</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="app">
	<h1>Tracking data for <span>{data.name}</span></h1>

	<div class="tracking-id">
		<div>
			<p>Tracking ID</p>
			<h3>{$page.params.id}</h3>
		</div>
		<button on:click={copyTrackingId}>Copy</button>
	</div>

	<section class="stats">
		<div>
			<canvas bind:this={sessionsAndVisitorsChart} />
		</div>
		<div>
			<canvas bind:this={sessionsAndVisitorsByHourChart} />
		</div>
		<div>
			<canvas bind:this={visitorsCountByBrowserChart} />
		</div>
		<div>
			<canvas bind:this={visitorsCountByOsChart} />
		</div>
		<div>
			<canvas bind:this={visitorsCountByDeviceChart} />
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
