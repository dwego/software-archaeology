# Case 001 — Silent Orbit

> Official solution and author notes.
> This file contains spoilers.

## Incident Summary

ORPHEUS-3 experienced two communication failures after the
communication subsystem was updated to Revision C.

Revision C was an engineering draft and had not been approved
for flight use.

## Root Cause

The fault-introducing change was commit `54d54dc`.

The commit updated two independent parts of the communication
subsystem according to Revision C.

### Telemetry

Revision B defined the frame as:

SYNC | TYPE | LENGTH | PAYLOAD | CHECKSUM

Revision C proposed:

SYNC | LENGTH | TYPE | PAYLOAD | CHECKSUM

The onboard firmware adopted Revision C while the operational
GDU-4 ground decoder remained on Revision B.

This caused TYPE and LENGTH to be interpreted incorrectly.

### Radio

The radio hardware register stores frequency in units of 10 kHz.

The original driver converted:

137400 kHz -> 13740

During the Revision C refactor, the public API was changed to
accept MHz and convert it to kHz.

However, the final conversion from kHz to 10 kHz register units
was removed.

The resulting value was written directly to a 16-bit register,
producing the incorrect register value observed during Ground
Test 042.

## Why Internal Testing Passed

The internal loopback connected the flight encoder to an onboard
diagnostic decoder built from the same software revision.

Both sides therefore understood the Revision C layout.

The loopback verified internal compatibility, but did not test
compatibility with GDU-4.

## Evidence

- `ARD-COMM-12 Revision B`
- `ARD-COMM-12 Revision C`
- `GSD-68-1021`
- `CHG-68-0820`
- `FSW-68-1105`
- `Ground Test 041`
- `Ground Test 042`
- commit `54d54dc`

## Corrective Action

Restore Revision B telemetry framing for ORPHEUS-3.

Restore the radio driver's conversion from kHz to 10 kHz hardware
register units.

Revision C should only be deployed after the corresponding ground
equipment and system-level validation are available.