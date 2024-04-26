# deschuler

> [!IMPORTANT]
> This crate was developed to use in the [financrr](https://github.com/financrr/financrr-app) projekt.
> It will be extended as needed.

An async, tokio based scheduling library for rust with a built-in cron builder.
It uses [croner](https://github.com/Hexagon/croner-rust) for the most available cron features.

## Features

- [x] Cron builder
- [x] Timezone support
- [x] Async scheduling
- [x] Sync scheduling
- [ ] Error handler
- [ ] Persistent scheduling
- [ ] Event/Notification system
- [ ] shared data
- [ ] support for crono and time
- [ ] more scheduler (pure std, async-stc, etc.)
- [ ] monitoring

## My goals

The goal is to create a feature-rich scheduling library that is easy to use.
<br>
It should be very flexible. The "first" implementation uses tokio, but I want to make it possible that we supply more
backends in the future.
