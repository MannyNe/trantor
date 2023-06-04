export const ssr = false;

import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { CountPathsSchema, SourcesSchema, TrackingDataSchema } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const trackingRes = await fetch(`/admin/trackings/${params.id}`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const trackingData = await trackingRes.json();
	const tracking = TrackingDataSchema.parse(trackingData);

	const sourcesRes = await fetch(`/admin/trackings/${params.id}/sources`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const sourcesData = await sourcesRes.json();
	const { sources } = SourcesSchema.parse(sourcesData);

	const pathsRes = await fetch(`/admin/trackings/${params.id}/paths`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const pathsData = await pathsRes.json();
	const { paths } = CountPathsSchema.parse(pathsData);

	return {
		tracking,
		sources,
		paths
	};
}) satisfies PageLoad;
