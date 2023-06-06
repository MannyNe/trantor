import type { PageLoad } from './$types';
import { getAuthToken } from '$lib/auth';
import { SourcesSchema, CountPathsSchema, CountTitlesSchema } from '$lib/schema';

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

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
		sources,
		paths,
		titles
	};
}) satisfies PageLoad;
