#pragma once

#include "wpi/hal/DriverStationTypes.h"
#include "wpi/hal/Types.h"

extern "C" {
    void _HALShim_InitializeDashboardOpMode();
    void _HALShim_SetDashboardOpModeOptions(const HAL_OpModeOption* optionsPtr, size_t optionsCount);
    void _HALShim_StartDashboardOpMode();
    void _HALShim_EnableDashboardOpMode();
    int64_t _HALShim_GetDashboardSelectedOpMode(HAL_RobotMode robotMode);

    HAL_Status _HALShim_MakeError(HAL_Status status, const char* messagePtr, size_t messageLen);
    HAL_Status _HALShim_MakeErrorIndexOutOfRange(
        HAL_Status status,
        const char* messagePtr,
        size_t messageLen,
        int32_t minimum,
        int32_t maximum,
        int32_t channel
    );
    HAL_Status _HALShim_MakeErrorPreviouslyAllocated(
        HAL_Status status,
        const char* messagePtr,
        size_t messageLen,
        int32_t channel,
        const char* previousAllocation,
        size_t previousAllocationLen
    );
}
