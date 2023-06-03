import wretch from 'wretch';

const API_URL = 'http://localhost:3030/admin';

export function authenticate({
	userId,
	secretCode,
	onSuccess,
	onError
}: {
	userId: string;
	secretCode: string;
	onSuccess: () => void;
	onError: () => void;
}) {
	const token = btoa(`${userId}:${secretCode}`);

	wretch(`${API_URL}/authenticate`)
		.auth(`Basic ${token}`)
		.post()
		.res((res) => {
			if (res.ok) {
				onSuccess();
			} else {
				onError();
			}
		})
		.catch(onError);
}

type Tracking = {
	id: string;
	name: string;
	created_at: number;
	visitor_count: number;
	sessions_count: number;
	events_count: number;
	sources_count: number;
};

type ListTrackings = {
	trackings: Tracking[];
};

export async function listTrackings({
	userId,
	secretCode
}: {
	userId: string;
	secretCode: string;
}) {
	const token = btoa(`${userId}:${secretCode}`);
	const trackings = await wretch(`${API_URL}/trackings`)
		.auth(`Basic ${token}`)
		.get()
		.json<ListTrackings>();

	return trackings;
}

export async function createTracking({
	userId,
	secretCode,
	name
}: {
	userId: string;
	secretCode: string;
	name: string;
}) {
	const token = btoa(`${userId}:${secretCode}`);

	await wretch(`${API_URL}/trackings`).auth(`Basic ${token}`).json({ name }).post();
}
