import { goto } from '$app/navigation';
import { writable } from 'svelte/store';
import { redirect } from '@sveltejs/kit';

import { getAuthState, authenticate, setAuthState } from './auth';

type AuthState = {
	userId: string;
	secretCode: string;
};

export const authState = writable<AuthState | null>(getAuthState());

authState.subscribe((v) => {
	setAuthState(v);

	if (v === null) {
		goto('/login');
	}
});

export async function initAuth() {
	const localStorageAuthState = getAuthState();

	if (localStorageAuthState === null) return;

	await authenticate(localStorageAuthState.userId, localStorageAuthState.secretCode, {
		onSuccess: async () => {
			authState.set(localStorageAuthState);
		},
		onError: async () => {
			authState.set(null);
			throw redirect(303, '/login');
		}
	});
}
