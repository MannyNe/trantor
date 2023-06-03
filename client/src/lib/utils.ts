type AuthState = {
	userId: string;
	secretCode: string;
};

const AUTH_STATE_KEY = 'AUTH_STATE';

export function getAuthState(): AuthState | null {
	const stored = localStorage.getItem(AUTH_STATE_KEY) || 'null';
	return JSON.parse(stored) as AuthState | null;
}

export function getAuthToken(): string | null {
	const auth = getAuthState();

	if (!auth) {
		return null;
	}

	return btoa(`${auth.userId}:${auth.secretCode}`);
}

export function setAuthState(authState: AuthState | null) {
	localStorage.setItem(AUTH_STATE_KEY, JSON.stringify(authState));
}
