---
tags: cyber, article
crystal-type: article
crystal-domain: cyber
alias: gregorian debt
---
# legacy time

legacy time is the stack humanity counts days with today: the Gregorian calendar, the seven-day week, the 24-hour clock, UTC with its leap seconds and the time zones. each layer comes from a different era and a different authority, glued together without a common rule. it runs on habit and charges every person, every ledger and every machine a permanent conversion tax.

## months

| fault | detail |
|---|---|
| four lengths | 28, 29, 30 and 31 days, fixed by Roman politics; people memorise them with knuckles and rhymes |
| names out of place | september, october, november, december mean seventh to tenth and sit ninth to twelfth: the names date from a year that opened in March |
| rulers in the calendar | july and august were Quintilis and Sextilis until they were renamed for Julius Caesar and Augustus |
| a month detached from the moon | the word month comes from the [[moon]], yet Gregorian months follow no phase: the [[synodic month]] lasts 29.53059 days and drifts through every date |
| unequal quarters | 90, 91, 92 and 92 days: every quarter-on-quarter comparison carries a built-in error |

## years

| fault | detail |
|---|---|
| a start without a sky event | January 1 is the day Roman consuls took office from 153 BC; the solstice falls ten days earlier |
| a patched leap rule | every fourth year, except centuries, except every fourth century: a mean year of 365.2425 days against a tropical year of about 365.2422, one day of drift in roughly three thousand years |
| weeks overflow the year | 365 days are 52 weeks and 1 day, so every date changes its weekday each year and printed calendars expire; the pattern closes after 400 years, 146 097 days, 20 871 weeks |
| a missing zero | 1 BC is followed by AD 1, so every span across the era is off by one; astronomers insert a year 0 |
| staggered adoption | Catholic states switched in 1582 and dropped 10 days, Britain in 1752 dropped 11, Russia in 1918 dropped 13, Greece followed in 1923; the October revolution happened on 7 November |
| seasons of one hemisphere | the calendar's seasons are northern: in Bali, Australia, Argentina and South Africa, July is winter |

## days and seconds

| fault | detail |
|---|---|
| mixed bases | 24 hours from Egypt, 60 minutes and 60 seconds from Babylon, milliseconds in base 10 |
| two clocks for one second | since 1967 the SI second counts 9 192 631 770 periods of caesium-133; the day follows Earth's rotation, which slows by about two milliseconds per century |
| leap seconds | UTC took 27 leap seconds between 1972 and 2016, each announced about six months ahead; the 2012 and 2016 insertions took down servers at Reddit and Cloudflare; in 2022 the CGPM decided to stop adding them by 2035 |
| political zones | about forty UTC offsets from −12:00 to +14:00, some at half and quarter hours (India +5:30, Nepal +5:45); China runs one zone across five geographic ones |
| daylight saving | dozens of countries move the clock an hour twice a year: one day of 23 hours and one of 25 in the same year |
| ambiguous notation | 03/04/2026 reads as March 4 in the US and 3 April in Europe; ISO 8601 fixes the order as 2026-04-03, everyday writing keeps the local habit |

## the cost

every layer is a convention a machine has to emulate: a timezone database released several times a year, a leap second table, locale formats, holiday rules. people pay with plans cut into irregular months and with a sense of time detached from the sky they live under.

## the alternative

| need | answer |
|---|---|
| one count on every machine | [[unix time]]: seconds since 1970-01-01 00:00 UTC, every day exactly 86 400 of them |
| one rhythm every observer sees | the [[moon]]: [[lunar machine time]] counts the lunar day from the synodic month |
| a year that fits the land | [[cyberculture/calendar|calendar]]: thirteen months for the southern sky, the year opens in July |

see [[cyberculture/calendar|calendar]] · [[lunar machine time]] · [[mt]]
