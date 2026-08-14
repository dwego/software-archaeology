#ifndef ORPHEUS_PROTOCOL_H
#define ORPHEUS_PROTOCOL_H

#include <stdint.h>

#define ORPHEUS_SYNC_BYTE 0xA7
#define ORPHEUS_MAX_PAYLOAD 32

enum telemetry_type {
    TELEMETRY_STATUS   = 0x01,
    TELEMETRY_POWER    = 0x02,
    TELEMETRY_THERMAL  = 0x03,
    TELEMETRY_ATTITUDE = 0x04
};

typedef struct {
    uint8_t type;
    uint8_t length;
    uint8_t payload[ORPHEUS_MAX_PAYLOAD];
} telemetry_packet_t;

#endif