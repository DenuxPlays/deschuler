# deschuler

> [!IMPORTANT]
> NOT MAINTAINED ANYMORE!!!

I've built this library to be a scheduler that uses croner as it's backend.  
But now the [tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler) crate also uses croner so there is no need to built a new library.  

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
- [ ] support for chrono and time
- [ ] support for log crate
- [ ] more scheduler (pure std, async-stc, etc.)
- [ ] monitoring

## My goals

The goal is to create a feature-rich scheduling library that is easy to use.  
It should be very flexible. The "first" implementation uses tokio, but I want to make it possible that we supply more
backends in the future.

Each implementation can support different features. So that when a new feature is added not all backends have to be
updated.
