import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { TrackingCounts } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const res = await fetch(`/admin/trackings/${params.id}/counts`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const sourcesData = await res.json();

	return TrackingCounts.parse(sourcesData);
}) satisfies PageLoad;
