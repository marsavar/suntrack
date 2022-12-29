# 🌄 Suntrack

`suntrack` is a command line tool that lets you know when the sun rises and sets in a given city.

### Installation
```shell
cargo install suntrack
```

### Usage
```
A command line tool that lets you know when the sun rises and sets in a city on any given date

Usage: suntrack [OPTIONS] <CITY>

Arguments:
  <CITY>  The name of the city

Options:
  -l, --local        Use local time instead of UTC
  -d, --date <DATE>  Show times for a given date, which must be provided in the format YYYY-MM-DD. Defaults to today if not provided
  -h, --help         Print help information
```

Example:
```shell
suntrack stockholm
```

```
🌍 City 	Stockholm (SE)
🌄 Sunrise	2022-12-29 07:45:00 UTC
🌆 Sunset	2022-12-29 13:54:03 UTC
```

If more than one match is found, `suntrack` lets you disambiguate with multiple choices. For instance:

Example:
```shell
suntrack london
```

```
Found 6 options for london:
1) London (US)
2) London Grove (US)
3) Londonderry (US)
4) London Colney (GB)
5) London (CA)
6) London (GB)
Input number:
6
🌍 City 	London (GB)
🌄 Sunrise	2022-12-29 08:06:11 UTC
🌆 Sunset	2022-12-29 15:58:29 UTC
```

### Local time
If you want to use your local system time instead of UTC, you can use the flag `-l` or `--local`.
For example, if you were based in New South Wales and wanted to check sunrise and sunset times for Sydney, the times would be displayed using UTC+11.

```shell
suntrack -l sydney
```

```
🌍 City 	Sydney (AU)
🌄 Sunrise	2022-12-29 05:45:10 +11:00
🌆 Sunset	2022-12-29 20:08:23 +11:00
```

### Overriding the date
`suntrack` defaults to today's sunrise and sunset times. However, you can use the `-d <DATE>` or `--date <DATE>` flag to pick an arbitrary date in the format `YYYY-MM-DD`. 

```shell
suntrack -d 2025-08-15 'London (GB)'
```
```
🌍 City 	London (GB)
🌄 Sunrise	2025-08-15 04:46:14 UTC
🌆 Sunset	2025-08-15 19:23:59 UTC
```

### Credits
`suntrack` relies on the basic version of the [World Cities Database](https://simplemaps.com/data/world-cities), which is licensed under Creative Commons Attribution 4.0.