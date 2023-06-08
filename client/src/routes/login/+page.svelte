<script lang="ts">
	import { goto } from '$app/navigation';
	import { authenticate } from '$lib/auth';
	import { authState } from '$lib/auth.store';

	let errorDisplay = '';

	async function handleSubmit(event: Event) {
		const formData = new FormData(event.target as HTMLFormElement);

		const userId = formData.get('userId')! as string;
		const secretCode = formData.get('secretCode')! as string;

		await authenticate(userId, secretCode, {
			onSuccess: async () => {
				authState.set({ userId, secretCode });
				await goto('/');
			},
			onError: async () => {
				errorDisplay = 'Something Went Wrong';
			}
		});
	}
</script>

<svelte:head>
	<title>Login | Trantor</title>
</svelte:head>

<section>
	<div>
		<div class="form-container">
			<h1>Welcome to Trantor!</h1>
			<form on:submit|preventDefault={handleSubmit}>
				{errorDisplay}
				<div>
					<input required name="userId" type="text" placeholder="User ID" size="30" />
				</div>
				<div>
					<input required name="secretCode" type="text" placeholder="Secret Code" size="30" />
				</div>
				<div>
					<button type="submit">Login</button>
				</div>
			</form>
		</div>
	</div>

	<div class="img-container" />
</section>

<style>
	section {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		height: 100%;
	}

	section > div {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.form-container {
		padding: 1rem;
		width: fit-content;
	}

	h1 {
		font-family: 'Press Start 2P', cursive;
		font-size: 2rem;
		margin-bottom: 2rem;
	}

	form {
		display: flex;
		flex-direction: column;
		justify-content: center;
		gap: 1rem;
	}

	input {
		padding: 1rem;
		border: 1px solid #000;
		background-color: transparent;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);
		font-family: monospace;
		font-size: 1.2rem;
	}

	button {
		border: none;
		padding: 1rem;
		box-shadow: 5px 6px rgba(0, 0, 0, 0.5);
		background-color: #000;
		cursor: pointer;
		font-size: 1.2rem;
		font-family: 'Press Start 2P', cursive;
		color: white;
	}

	button:hover,
	button:focus {
		background-color: #aaa;
	}

	button:active {
		transform: scale(0.8);
	}

	.img-container {
		background-image: url('/illustrations/login-page.svg');
		background-repeat: no-repeat;
		background-position: center;
	}

	@media (max-width: 1000px) {
		.img-container {
			background-size: 200%;
		}
	}

	@media (max-width: 800px) {
		section {
			grid-template-columns: 1fr;
		}

		.img-container {
			display: none;
		}
	}

	@media (max-width: 500px) {
		h1 {
			text-align: center;
			line-height: 3rem;
		}

		.form-container {
			width: 100%;
		}

		input {
			width: 100%;
		}

		button {
			width: 100%;
		}
	}
</style>
