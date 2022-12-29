# 🌄 Suntrack

`suntrack` is a command line tool that lets you know when the sun rises and sets in a given city.

```shell
cargo install suntrack
```

### Usage
```shell
suntrack <CITY> [--local]
```

Example:
```shell
suntrack stockholm
```

```
🌍 Stockholm (SE)
🌄 Sunrise: 2022-12-29 07:45:00 UTC
🌆 Sunset: 2022-12-29 13:54:03 UTC
```

If more than one match is found, `suntrack` lets you disambiguate with multiple choice. For instance:

Example:
```shell
suntrack london
```

```
Found 6 options for london:
1) London Colney (GB)
2) London (CA)
3) London (GB)
4) London (US)
5) Londonderry (US)
6) London Grove (US)
Input number: 
3
🌍 London (GB)
🌄 Sunrise: 2022-12-29 08:06:11 UTC
🌆 Sunset: 2022-12-29 15:58:29 UTC
```