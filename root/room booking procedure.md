---
tags: cyberia
crystal-type: process
crystal-domain: cyberia
stake: 4707732808238825
diffusion: 0.0002286219884053515
springs: 0.000030231185661009345
heat: 0.00009822083574902732
focus: 0.00014302451705078473
gravity: 1
density: 0
---
#### goal:
- ensure booking accuracy by confirming all essential details.

#### procedure

- request the full names of all guests – collect names for the booking record.
- ask for the number of guests – confirm the total number of occupants.
- request the check-in date – ask when the guest plans to arrive.
- confirm the length of stay – ask how many nights the guest is booking.
- calculate and confirm the check-out date – determine the departure date and verify with the guest.
- ask for the estimated check-in time (UTC) – guests should check in no earlier than 06:00 UTC to align with 14:00 Bali time (WITA, UTC+8).
- confirm the check-out time (UTC) – check-out must be no later than 04:00 UTC to align with 12:00 Bali time (WITA, UTC+8).

#### required details

- full names of guests
- number of guests
- check-in date
- number of nights
- check-out date

#### example interaction

- agent: please provide the full names of all guests.
- guest: john smith and jane smith.
- agent: how many guests will be staying?
- guest: 2 adults.
- guest: april 10.
- agent: how many nights will you stay?
- guest: 5 nights.
- agent: confirming: check-in on april 10, check-out on april 15. is that correct?
- agent: what time do you expect to check in? please note that check-in is available from 06:00 utc (14:00 bali time).
- guest: we will arrive at 07:30 utc.
- agent: noted. your check-in time is 07:30 utc. see you then!
- agent: please note that check-out must be completed by 04:00 utc (12:00 bali time). will that work for you?
- guest: yes, that’s fine.
- agent: great! your booking is confirmed.

⚠ important: a booking is not considered complete without confirming all required details.