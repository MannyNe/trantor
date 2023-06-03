export const ssr = false;

import type { PageLoad } from './$types';
import { listTrackings } from '$lib/api';
import { redirect } from '@sveltejs/kit';
import { getAuthState } from '$lib/utils';

export const load = (async () => {
	const auth = getAuthState();

	if (!auth) {
		throw redirect(300, '/login');
	}

	const trackings = await listTrackings({
		userId: auth.userId,
		secretCode: auth.secretCode
	});

	return trackings;
}) satisfies PageLoad;
