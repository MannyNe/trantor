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
