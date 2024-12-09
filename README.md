# backlight_control_rs

[![Static Badge](https://img.shields.io/badge/Crates.io-orange?style=flat)](https://crates.io/crates/backlight_control_rs)
![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FDOD-101%2Fbacklight_control_rs%2Frefs%2Fheads%2Fmaster%2FCargo.toml&query=package.version&label=Version&color=rgb(20%2C%2020%2C%2020))

Backlight control is a re-written version of [backlight_control](https://github.com/Hendrikto/backlight_control/) with a few key improvements.

## Why re-write this? 

There are a few reasons: 

1. The original backlight_control sets the backlight dir at compile time in the make file. The problem with this is that it makes it impossible to be cached in package repositories. backlight_control_rs determines this information at runtime, meaning caching is possible.

2. backlight_control_rs comes with a library, so that other projects can easily adjust the backlight brightness as well. 

3. backlight_control_rs has made a few QOL changes compared to the original: better help message, `--stats` option, absolute and relative adjustments

4. backlight_control doesn't work on NixOs due to how it's written. backlight_control_rs addresses this.

## Why use this over something like `light`?

[light](https://gitlab.com/dpeukert/light) is also a good alternative, however I personally very much enjoyed the high degree of simplicity of backlight_control. Light offers many more features than I ever needed, hence backlight_control was the clear choice. It did what I needed and nothing more. 

And backlight_control_rs has all the simplicity of backlight_control, without any of the troubles.

As for other options, they either fall into the same category as light in having too much, or needing X-server to work.

## Usage

```sh
backlight_control_rs +50 # increase brightness by 50
backlight_control_rs -50 # reduce brightness by 50
backlight_control_rs 50 # set brightness to 50
```

NOTE: These values are absolute. They do not adjust as `n%` as with backlight_control. 
To do this, append a `%`:

```sh
backlight_control_rs +50% # increase brightness by 50%
backlight_control_rs -50% # reduce brightness by 50%
backlight_control_rs 50% # set brightness to 50%
```

You can run with `-s` to print information about the backlight, rather than making adjustments.

## Installing

You can use `cargo install`, however, this method is less desirable than installing via your package manager.

### AUR

WIP

## Note on permissions

The original backlight_control used permissions to allow anyone to run the binary as root. 
This made it possible to change the backlight brightness without any further configuration.
backlight_control_rs takes a different aproach:

### udev rules

This is what most other programs do to handle this issue. 

By using a udev rule to change the ownership of the `brightness` file to the `video` group any user in that group, can change the backlight brightness.

<!-- On NixOs setting this rule is handled for you by setting `programs.backlight_control_rs.enable = true`. --> 

This is also the reason why you should use a package manager rather than cargo install, sine the permissions won't be handled for you.

## License 

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](https://opensource.org/license/MIT)

at your option.

Its inspiration comes in no small part from [backlight_control](https://github.com/Hendrikto/backlight_control) and [light](https://gitlab.com/dpeukert/light).

