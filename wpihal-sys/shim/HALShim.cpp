/**
 * This exists to bridge a bunch of specific C++ methods to bindgen-friendly signatures. 
 * 
 */

#include "HALShim.h"
#include <span>
#include <string_view>
#include <cstdint>
#include "wpi/hal/DashboardOpMode.hpp"
#include "wpi/hal/ErrorHandling.hpp"

extern "C" {
    void _HALShim_InitializeDashboardOpMode() {
        wpi::hal::InitializeDashboardOpMode();
    }
    void _HALShim_SetDashboardOpModeOptions(const HAL_OpModeOption* optionsPtr, size_t optionsCount) {
        std::span<const HAL_OpModeOption> options{optionsPtr, optionsCount};
        wpi::hal::SetDashboardOpModeOptions(options);
    }
    void _HALShim_StartDashboardOpMode() {
        wpi::hal::StartDashboardOpMode();
    }
    void _HALShim_EnableDashboardOpMode() {
        wpi::hal::EnableDashboardOpMode();
    }
    int64_t _HALShim_GetDashboardSelectedOpMode(HAL_RobotMode robotMode) {
        return wpi::hal::GetDashboardSelectedOpMode(robotMode);
    }

    HAL_Status _HALShim_MakeError(HAL_Status status, const char* messagePtr, size_t messageLen) {
        std::string_view message{messagePtr, messageLen};
        return wpi::hal::MakeError(status, message);
    }
    HAL_Status _HALShim_MakeErrorIndexOutOfRange(
        HAL_Status status,
        const char* messagePtr,
        size_t messageLen,
        int32_t minimum,
        int32_t maximum,
        int32_t channel
    ) {
        std::string_view message{messagePtr, messageLen};
        return wpi::hal::MakeErrorIndexOutOfRange(status, message, minimum, maximum, channel);
    }
    HAL_Status _HALShim_MakeErrorPreviouslyAllocated(
        HAL_Status status,
        const char* messagePtr,
        size_t messageLen,
        int32_t channel,
        const char* previousAllocationPtr,
        size_t previousAllocationLen
    ) {
        std::string_view message{messagePtr, messageLen};
        std::string_view previousAllocation{previousAllocationPtr, previousAllocationLen};
        return wpi::hal::MakeErrorPreviouslyAllocated(status, message, channel, previousAllocation);
    }

}
