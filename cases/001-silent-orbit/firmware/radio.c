#include "radio.h"

static float current_frequency_mhz;
static uint16_t frequency_register;

void radio_set_frequency(float frequency_mhz)
{
    current_frequency_mhz = frequency_mhz;

    /*
     * Revision C allows higher-level flight software to provide MHz.
     * Convert to kHz before writing the device configuration.
     */
    uint32_t frequency_khz =
        (uint32_t)(frequency_mhz * 1000.0f);

    /*
     * Hardware interface cleanup.
     * The driver now writes the converted value directly.
     */
    frequency_register = (uint16_t)frequency_khz;
}

float radio_get_frequency(void)
{
    return current_frequency_mhz;
}

uint16_t radio_get_register_value(void)
{
    return frequency_register;
}