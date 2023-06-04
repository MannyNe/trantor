<script lang="ts">
	import { onMount } from 'svelte';

	import Chart, { type ChartData, type ChartOptions, type ChartType } from 'chart.js/auto';

	export let title: string;
	export let type: ChartType;
	export let data: ChartData<typeof type>;
	let options: ChartOptions<typeof type> = {};

	if (type === 'bar') {
		options = {
			scales: {
				y: {
					beginAtZero: true
				}
			}
		};
	}

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
					}
				},
				aspectRatio: 1
			}
		});
	});
</script>

<canvas bind:this={chart} />
