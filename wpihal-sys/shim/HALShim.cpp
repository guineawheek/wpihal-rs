/**
 * This exists to bridge a bunch of specific C++ methods to bindgen-friendly signatures. 
 * 
 */

#include "HALShim.h"
#include <span>
#include <string_view>
#include <cstdint>
#include "wpi/hal/DashboardOpMode.hpp"

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

}
