import { browser } from '$app/environment';
import { redirect } from '@sveltejs/kit';

type AuthState = {
	userId: string;
	secretCode: string;
};

const AUTH_STATE_KEY = 'AUTH_STATE';

export function getAuthState() {
	if(!browser) return null; 
	const stored = localStorage.getItem(AUTH_STATE_KEY);
	if (stored === null) return null;
	else return JSON.parse(stored) as AuthState;
}

export function getAuthToken() {
	const auth = getAuthState();

	if (auth === null) {
		throw redirect(303, '/login');
	}

	return btoa(`${auth.userId}:${auth.secretCode}`);
}

export function setAuthState(authState: AuthState | null) {
	localStorage.setItem(AUTH_STATE_KEY, JSON.stringify(authState));
}

export async function authenticate(
	userId: string,
	secretCode: string,
	{
		onSuccess,
		onError
	}: {
		onSuccess: () => Promise<void>;
		onError: () => Promise<void>;
	}
) {
	const token = btoa(`${userId}:${secretCode}`);

	const res = await fetch('/admin/authenticate', {
		method: 'POST',
		headers: {
			Authorization: `Basic ${token}`
		}
	});

	if (res.ok) {
		await onSuccess();
	} else {
		await onError();
	}
}
