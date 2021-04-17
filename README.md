# LOKI BMS
* What it is: a free, open-source battle management scope for use with DCS World, similar to LOTATC
* What it is not: a comprehensive piece of air control software (yet)

## Philosophy
LOKI BMS is designed to be a simple, easy to use scope for effectively managing air traffic and combat situations in a **SIMULATOR** environment. Its goal is to provide controllers with a single solution for command and control, and eventually scenario management without depending on other pieces of software.

With that in mind, it would not be unreasonable for LOKI to work as a source or secondary interface with other pieces of software, provided LOKI can still run independently.

## Current Status
LOKI is currently in an **ALPHA** state. This means that it is not yet fully functional and may have significant bugs when features are added.

There are currently no release versions of LOKI available for download.

## Features
LOKI will feature a variety of configuration options to fine-tune detection and tracking behavior at the server. A server may take in raw sensor data from a number of sources, provided they include basic data like coordinates, object type, and faction (if required).

The three major tracking methods LOKI is aiming to provide are:
* Truth-Data Tracking (raw data is used to maintain tracks with no ambiguity)
* Automatic Tracking (server initiates and maintains track files, users ID and update information)
* Semi-Automatic Tracking (users initiate and maintain track files)

LOKI will also look to support a blended tracking solution based on whether platforms are capable of datalinked communications.

In the future, LOKI may integrate more closely with applications like [LOTATC](https://www.lotatc.com/) and [SRS](https://dcssimpleradio.com/) to utilize SRS's IFF features, in order to improve realism and tracking.

LOKI is written in [Rust](https://www.rust-lang.org/), using the [iced](https://github.com/hecrj/iced) GUI framework.

## FAQ
* WIP