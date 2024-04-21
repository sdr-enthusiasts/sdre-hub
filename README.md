# SDR-Enthusiasts Hub

## Introduction

SDR-E Hub is the spiritual successor to [ACARS Hub](https://github.com/SDR-Enthusiasts/docker-acarshub).

When I started the ACARS Hub project years ago I had no idea where it would go. It started small; just a very simple interface to read raw JSON output from ACARS and VDLM decoders. Quickly, with input from the SDR-E community, and my own wants, it grew into a full fledged web application that could show ACARS/VDLM/HFDL/Satellite Based ACARS, as well as a pair it with ADS-B data and store data to view later. However, because it started life as a simple JSON reader, it was never designed to be a full fledged web application. It was a hack, and it showed. And, of course, at the time my skillset with programming was much less than it is now; not that it's terribly impressive now, but it's better than it was.

So, I decided to start from scratch. I wanted to build the replacement for ACARS Hub that was designed from the ground up to be everything we wanted it to be, and even things we haven't yet imagined. I wanted to build a web application that was designed to be modular, and easily extensible. I wanted to build a web application that was designed to be easy to use, and easy to deploy.

Thus, SDR-E Hub was born. I considered keeping this under the ACARS Hub name, but I felt that it was time to move on. ACARS Hub was a great project, and I learned a lot from it, but it was time to move on. This is a from scratch, ground up idea that is designed to be everything ACARS Hub was, and more.

## What is SDR-E Hub?

SDR-E Hub is a visual interface, both a web application and/or an application you can run on your own computer, for reading and analyzing data from various SDR decoders. This includes ACARS, VDLM, HFDL, ADS-B, and more.

## But wait, doesn't tar1090 and virtualradarserver do this?

Yes, but also no. This is not a replacement for tar1090 in any way. tar1090 is much more optimized for displaying real time ADS-B data. This is much closer to a replacement for virtualradarserver, but with a focus on more than just ADS-B data as well as being actively developed and easier to configure.

## Migrating from ACARS Hub

Sorry, there is no migration path from ACARS Hub to SDR-E Hub. The data that ACARS Hub saves is so incomplete that there really isn't a point in migrating anything over.
