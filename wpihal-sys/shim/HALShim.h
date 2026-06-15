#pragma once

#include "wpi/hal/DriverStationTypes.h"
#include "wpi/hal/Types.h"

extern "C" {
    void _HALShim_InitializeDashboardOpMode();
    void _HALShim_SetDashboardOpModeOptions(const HAL_OpModeOption* optionsPtr, size_t optionsCount);
    void _HALShim_StartDashboardOpMode();
    void _HALShim_EnableDashboardOpMode();
    int64_t _HALShim_GetDashboardSelectedOpMode(HAL_RobotMode robotMode);

}
