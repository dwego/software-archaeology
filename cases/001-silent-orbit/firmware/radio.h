#ifndef ORPHEUS_RADIO_H
#define ORPHEUS_RADIO_H

#include <stdint.h>

void radio_set_frequency(uint32_t frequency_khz);
uint32_t radio_get_frequency(void);
uint16_t radio_get_register_value(void);

#endif