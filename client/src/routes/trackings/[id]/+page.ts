export const ssr = false;

import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { SourcesSchema, TrackingDataSchema } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const trackingsRes = await fetch(`/admin/trackings/${params.id}`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const trackingsData = await trackingsRes.json();
	const trackings = TrackingDataSchema.parse(trackingsData);

	const sourcesRes = await fetch(`/admin/trackings/${params.id}/sources`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const sourcesData = await sourcesRes.json();
	const sources = SourcesSchema.parse(sourcesData);

	return {
		trackings,
		sources
	};
}) satisfies PageLoad;
