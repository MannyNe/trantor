<script lang="ts">
	import { onMount } from 'svelte';

	import Chart, { type ChartData, type ChartOptions, type ChartType } from 'chart.js/auto';
	import autocolors from 'chartjs-plugin-autocolors';

	export let title: string;
	export let type: ChartType;
	export let data: ChartData<typeof type>;
	export let options: ChartOptions<typeof type> | {} = {};

	let chart: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D;

	onMount(() => {
		ctx = chart.getContext('2d')!;

		new Chart(ctx, {
			type,
			data,
			options: {
				...options,
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
						text: title
					},
					autocolors
				},
				aspectRatio: 1
			}
		});
	});
</script>

<canvas bind:this={chart} />
