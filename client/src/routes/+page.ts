export const ssr = false;

import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { TrackingsSchema } from '$lib/schema';

export const load = (async () => {
	const authToken = getAuthToken();

	const res = await fetch('/admin/trackings', {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const data = await res.json();

	return TrackingsSchema.parse(data);
}) satisfies PageLoad;
