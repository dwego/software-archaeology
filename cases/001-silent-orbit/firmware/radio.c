#include "radio.h"

static uint32_t current_frequency_khz;
static uint16_t frequency_register;

void radio_set_frequency(uint32_t frequency_khz)
{
    current_frequency_khz = frequency_khz;

    /*
     * Hardware stores the carrier frequency in 10 kHz units.
     * Keep this conversion inside the driver.
     *
     * RHC — 1968-08-19
     */
    frequency_register = (uint16_t)(frequency_khz / 10);
}

uint32_t radio_get_frequency(void)
{
    return current_frequency_khz;
}

uint16_t radio_get_register_value(void)
{
    return frequency_register;
}