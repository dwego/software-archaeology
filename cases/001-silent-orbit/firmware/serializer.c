#include "serializer.h"

size_t serialize_packet(
    const telemetry_packet_t *packet,
    uint8_t *output
) {
    if (packet->length > ORPHEUS_MAX_PAYLOAD) {
        return 0;
    }

    size_t index = 0;
    uint8_t checksum = 0;

    output[index++] = ORPHEUS_SYNC_BYTE;

    /*
     * Updated to match ARD-COMM-12 Revision C.
     */
    output[index++] = packet->length;
    output[index++] = packet->type;

    checksum += packet->type;
    checksum += packet->length;

    for (uint8_t i = 0; i < packet->length; i++) {
        output[index++] = packet->payload[i];
        checksum += packet->payload[i];
    }

    output[index++] = checksum;

    return index;
}