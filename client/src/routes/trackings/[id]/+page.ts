export const ssr = false;

import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { TrackingDataSchema } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const res = await fetch(`/admin/trackings/${params.id}`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const data = await res.json();

	return TrackingDataSchema.parse(data);
}) satisfies PageLoad;
