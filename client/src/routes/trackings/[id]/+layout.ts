import type { LayoutLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { TrackingDataSchema } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const trackingRes = await fetch(`/admin/trackings/${params.id}`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const trackingData = await trackingRes.json();
	const tracking = TrackingDataSchema.parse(trackingData);

	return {
		tracking
	};
}) satisfies LayoutLoad;
