import { derived, writable } from 'svelte/store';
import { authenticate } from './api';
import { getAuthState, setAuthState } from './utils';

type AuthState = {
	userId: string;
	secretCode: string;
};
export const authState = writable<AuthState | null>(null);

const localStorageAuthState = getAuthState();

if (localStorageAuthState) {
	authenticate({
		userId: localStorageAuthState.userId,
		secretCode: localStorageAuthState.secretCode,
		onSuccess: () => authState.set(localStorageAuthState),
		onError: () => {
			console.log('error');
		}
	});
}

authState.subscribe((val) => {
	setAuthState(val);
});

export const isAuthenticated = derived(authState, ($authState) => {
	return !!$authState;
});
