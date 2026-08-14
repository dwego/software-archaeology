#ifndef ORPHEUS_RADIO_H
#define ORPHEUS_RADIO_H

#include <stdint.h>

#define ORPHEUS_PRIMARY_FREQUENCY_KHZ 137400U
#define ORPHEUS_BACKUP_FREQUENCY_KHZ  137850U
#define ORPHEUS_BEACON_FREQUENCY_KHZ  136950U

void radio_set_frequency(uint32_t frequency_khz);
uint32_t radio_get_frequency(void);
uint16_t radio_get_register_value(void);

#endif