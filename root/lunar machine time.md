---
tags: cyber, cyberia
alias: lmt, lunar timestamp
crystal-type: measure
crystal-domain: cyber
icon: "\U0001F319"
---
# Lunar Machine Time

the native [[calendar]] of the [[cybergraph]]. combines the oldest human time cycle (the [[moon]]) with the youngest ([[unix time]]). format:

```
DD.MM.YY
```

| Position | Meaning | Range | Source |
|----------|---------|-------|--------|
| DD | lunar day | 1–30 | day within the current [[synodic month]] |
| MM | lunar month | 1–13 | moon number within the machine year |
| YY | machine year | 0–… | years since [[unix time]] epoch (1970) |

example: `24.13.55` = lunar day 24, moon 13, year 55 [[mt]] (Gregorian ~December 2025)

## Why

Gregorian months are arbitrary divisions of a solar year with no astronomical signal. The [[moon]] is a physical oscillator: 29.53 days, visible to every observer on [[Earth]], invariant across cultures.

[[Machine time]] is the universal clock of computation: seconds since 1970-01-01 00:00:00 UTC. Every machine on the planet agrees on this number.

Lunar machine time unifies the two: biological rhythm (lunar cycle) and computational precision ([[unix time]] years). No cultural, religious, or political overlay — pure astronomy and pure machines.

## Computation

```
synodic_period = 29.53059 days
reference_new_moon = 2000-01-06 18:14 UTC (Julian Day 2451550.26)

For any UTC datetime:
  1. julian_day = days since -4713-11-24 12:00 UTC
  2. days_since_ref = julian_day - 2451550.26
  3. lunar_age = days_since_ref mod 29.53059
  4. DD = floor(lunar_age) + 1

For lunar month in year:
  1. year_start = January 1 of current MT year (1970 + YY)
  2. first_new_moon = nearest new moon on or after year_start
  3. MM = floor((julian_day - first_new_moon) / 29.53059) + 1

For machine year:
  YY = calendar_year - 1970
```

the [[synodic month]] is the period between two identical [[moon]] phases (new moon to new moon): 29 days, 12 hours, 44 minutes, 2.9 seconds

## Eras

| MT Year | Gregorian | Era |
|---------|-----------|-----|
| 0 | 1970 | [[unix time]] epoch — first machine second |
| 39 | 2009 | [[bitcoin]] genesis block — machines learn to prove time |
| 49 | 2019 | [[bostrom]] genesis — [[cybergraph]] begins |
| 56 | 2026 | now |

any event before year 0 is measured in [[before machines]] (negative MT years)

## Usage

lunar machine time is displayed in the [[cybergraph]] publisher for page creation and modification dates. it is the default timestamp format across all [[cyber]] interfaces.

see [[mt]], [[time]], [[time/history]], [[calendar]]