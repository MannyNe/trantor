import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import {
	CountPathsSchema,
	CountTitlesSchema,
	SourcesSchema,
	TrackingDataSchema
} from '$lib/schema';

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

	const titlesRes = await fetch(`/admin/trackings/${params.id}/titles`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const titlesData = await titlesRes.json();
	const { titles } = CountTitlesSchema.parse(titlesData);

	return {
		tracking,
		sources,
		paths,
		titles
	};
}) satisfies PageLoad;
