# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

We use an updated version of the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.
<br>
What we've changed:

- We use a nicer title format (Version x.x.x (DD.MM.YYYY)).
- We add `### BREAKING` to breaking changes to make them stand out more.
  <br>

We do this to make it easier to read when using `cargo-dist` to generate the release notes from the changelog.

## unreleased

## Version 0.4.0 (12.05.2024)

### Added

- added `Job::interrupt`
- added `Job::is_interrupted`
- added `Job::get_job`

### Breaking

- changed `Scheduler::schedule_job(&mut self, cron: Cron, task: Job)`
  to `Scheduler::schedule_job(&mut self, cron: Cron, task: Job)`

## Version 0.3.0 (29.04.2024)

### Added

- added `TokioScheduler::new` and `TokioScheduler::default`

## Version 0.2.0 (28.04.2024)

### Breaking

- replaced `chrono_tz::Tz` with `chrono::FixedOffset` in `CronSchedule` and `CronScheduleBuilder`

## Version 0.1.1 (22.04.2024)

### Fixed

- fixed cron builder when using dow and dom

## Version 0.1.0 (15.04.2024)

### Added

- initial release
