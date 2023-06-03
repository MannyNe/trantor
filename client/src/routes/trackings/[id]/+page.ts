import type { PageLoad } from './$types';
import { env } from '$env/dynamic/public';
import { getAuthToken } from '$lib/utils';

export const prerender = true;
export const ssr = false;

type WeekdayCount = {
	count: number | null;
	weekday: number | null;
};
type HourCount = {
	count: number | null;
	hour: number | null;
};

type OsCount = {
	count: number | null;
	os: string | null;
};

type BrowserCount = {
	count: number | null;
	browser: string | null;
};

type DeviceCount = {
	count: number | null;
	device: string | null;
};

type TrackingData = {
	name: string;
	session_count_by_weekday: WeekdayCount[];
	visitor_count_by_weekday: WeekdayCount[];
	session_count_by_hour: HourCount[];
	visitor_count_by_hour: HourCount[];
	visitor_count_by_os: OsCount[];
	visitor_count_by_browser: BrowserCount[];
	visitor_count_by_device: DeviceCount[];
};

export const load = (async ({ params, fetch }) => {
	const authToken = getAuthToken();

	const res = await fetch(`${env.PUBLIC_API_URL}/trackings/${params.id}`, {
		headers: {
			Authorization: `Basic ${authToken}`
		}
	});
	const data = await res.json();

	return data as TrackingData;
}) satisfies PageLoad;
