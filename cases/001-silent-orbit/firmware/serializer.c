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

    /* Keep GDU compatibility in mind when touching this layout. */
    output[index++] = packet->type;
    output[index++] = packet->length;

    checksum += packet->type;
    checksum += packet->length;

    for (uint8_t i = 0; i < packet->length; i++) {
        output[index++] = packet->payload[i];
        checksum += packet->payload[i];
    }

    output[index++] = checksum;

    return index;
}