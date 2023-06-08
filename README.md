# Trantor

![logo](./logo.png)

Trantor is a simple analytics server that focuses on websites made to market and test products or start-up ideas. Typically in this kind of websites, the most important information you want is how many people are visiting your website, how long they are staying, and how many of them are returning and you might also want to store some information about them to contact them late like their email. Trantor is built to do just that, it's a simple analytics server that you can deploy on **your own server** and use to track **your websites**.

## Features

- Tracks page views, unique visitors, and returning visitors
- Tracks user information like OS, browser, and device
- Tracks how long a user stays on a page, via timestamps sent to the server when a session is started and ended
- Creating multiple "trackings", where each "tracking" is a different website or product
- Creating a source to track where the user came from, i.e `?src=telegram` or `?src=twitter`
- A self hostable, solution that can be deployed from a single binary
- A lightweight dashboard to manage your trackings and view analytics, built with [Svelte](https://svelte.dev/) and [Svelte Kit](https://kit.svelte.dev/)
- A performant, scalable, and reliable backend built with [Rust](https://www.rust-lang.org/)

### ⚠️ **Trantor** is still in early development, and is not ready for production use yet, issues and pull requests are welcome

## Running the project

1. Clone the repository

```bash
git clone git@github.com:frectonz/trantor.git
```

2. Install the dependencies for the frontend dashboard

```bash
cd client
pnpm install
```

3. Build the frontend dashboard

```bash
pnpm build
```

4. Add a `DATABASE_URL` environment variable that contains a connection string to a postgres database

```bash
export DATABASE_URL="postgresql://postgres:password@localhost:5432/trantor"
```

(optional) I recommend creating a `.envrc` file in the root of the project and adding the `DATABASE_URL` variable there.
The `.envrc` file should look like this:

```bash
#!/bin/bash

export DATABASE_URL="postgresql://postgres:password@localhost:5432/trantor"
```

You can use this environment variables by running the following command:

```bash
source .envrc
```

5. Now you can run the rust backend server, remember running the rust backend expects the `DATABASE_URL` environment variable to be set and the frontend dashboard to be built and available in the `client/build` directory. Be sure to run the backend from the root of the project.

```bash
cargo run
```

6. To be able to login you need to create a user, you can do that by making a `POST` request to the `http://localhost:3030/admin/users` endpoint with a `secret_code` in the body of the request.

You can do that with [httpie](https://httpie.io/):

```bash
http POST localhost:3030/admin/users secret_code=secret_code
```

You can do that with [curl](https://curl.se/):

```bash
curl -X POST localhost:3030/admin/users -H 'Content-Type: application/json' -d '{"secret_code":"10005"}'
```

Anyway you make this request, the server will respond with a `user_id` and a `secret_code` that you can use to login to the dashboard.

```json
{
    "secret_code": "10004",
    "user_id": "01H2CV5RYD9AWTC12S1REEGTHZ"
}
```

7. Now you can visit the dashboard at <http://localhost:3030>. Where you can login with a `user_id` and `secret_code` combination.

8. Success 🎉, you should now be able to create trackings and view analytics.
