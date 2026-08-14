#ifndef ORPHEUS_RADIO_H
#define ORPHEUS_RADIO_H

#include <stdint.h>

#define ORPHEUS_PRIMARY_FREQUENCY_MHZ 137.400f
#define ORPHEUS_BACKUP_FREQUENCY_MHZ  137.850f
#define ORPHEUS_BEACON_FREQUENCY_MHZ  136.950f

void radio_set_frequency(float frequency_mhz);
float radio_get_frequency(void);
uint16_t radio_get_register_value(void);

#endif