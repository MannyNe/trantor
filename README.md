# Trantor
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-3-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

![logo](./logo.png)

Trantor is a simple analytics server that focuses on websites made to market and test products or start-up ideas. Typically in this kind of websites, the most important information you want is how many people are visiting your website, how long they are staying, and how many of them are returning and you might also want to store some information about them to contact them later like their email. Trantor is built to do just that, it's a simple analytics server that you can deploy on **your own server** and use to track **your websites**.

## Features

- Tracks page views, unique visitors, and returning visitors
- Tracks user information like OS, browser, and device
- Tracks how long a user stays on a page, via timestamps sent to the server when a session is started and ended
- Creating multiple "trackings", where each "tracking" is a different website or product
- Creating a source to track where the user came from, i.e `?src=telegram` or `?src=twitter`
- A self hostable, solution that can be deployed from a single binary
- A lightweight dashboard to manage your trackings and view analytics, built with [Svelte](https://svelte.dev/) and [Svelte Kit](https://kit.svelte.dev/)
- A performant, scalable, and reliable backend built with [Rust](https://www.rust-lang.org/)
- Configurable via a simple `config.toml` file

### ⚠️ **Trantor** is still in early development, and is not ready for production use yet, issues and pull requests are welcome

## Running the project locally

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

4. Create a `config.toml` file in the root of the project, using the following example

```toml
address = "127.0.0.1:3030"
database = "postgresql://postgres:password@localhost:5432/trantor"

# Uncomment the following to enable `https` support
# [https]
# cert_path = "/etc/letsencrypt/live/trantor.frectonz.tech/fullchain.pem"
# key_path = "/etc/letsencrypt/live/trantor.frectonz.tech/privkey.pem"
```

You will need a postgres database running and reachable at the address specified in the `config` file. Don't worry about the optional `https` options you, since you are running the server on your local machine you can use `http`.

5. Now you can run the rust backend server, Be sure to run the backend from the root of the project.

```bash
cargo run -- config.toml
```

6. To be able to login you need to create a user, you can do that by making a `POST` request to the `http://localhost:3030/admin/users` endpoint with a `secret_code` in the body of the request.

You can do that with [httpie](https://httpie.io/):

```bash
http POST localhost:3030/admin/users secret_code=10005
```

or you can use [curl](https://curl.se/):

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

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
     <td align="center" valign="top" width="14.28%"><a href="https://frectonz.tech"><img src="https://avatars.githubusercontent.com/u/53809656?v=4?s=100" width="100px;" alt="Fraol Lemecha"/><br /><sub><b>Fraol Lemecha</b></sub></a><br /><a href="https://github.com/frectonz/trantor/commits?author=frectonz" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://echo.dev.et/"><img src="https://avatars.githubusercontent.com/u/45893263?v=4?s=100" width="100px;" alt="Elias Amha"/><br /><sub><b>Elias Amha</b></sub></a><br /><a href="https://github.com/frectonz/trantor/commits?author=0xecho" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/MannyNe"><img src="https://avatars.githubusercontent.com/u/50913365?v=4?s=100" width="100px;" alt="Amann Negussie"/><br /><sub><b>Amann Negussie</b></sub></a><br /><a href="https://github.com/frectonz/trantor/commits?author=MannyNe" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->
