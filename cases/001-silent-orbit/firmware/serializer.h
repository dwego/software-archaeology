#ifndef ORPHEUS_SERIALIZER_H
#define ORPHEUS_SERIALIZER_H

#include <stddef.h>
#include <stdint.h>

#include "protocol.h"

size_t serialize_packet(
    const telemetry_packet_t *packet,
    uint8_t *output
);

#endif