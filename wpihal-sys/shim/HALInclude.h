#include "HALShim.h"

#include "wpi/hal/AddressableLED.h"
#include "wpi/hal/AddressableLEDTypes.h"
#include "wpi/hal/Alert.h"
#include "wpi/hal/AnalogInput.h"

#include "wpi/hal/CAN.h"
#include "wpi/hal/CANAPI.h"
#include "wpi/hal/CANAPITypes.h"
#include "wpi/hal/CANBusMap.h"

#include "wpi/hal/Counter.h"
#include "wpi/hal/CTREPCM.h"
// DashboardOpMode.hpp is part of the HALSHim
#include "wpi/hal/DIO.h"
#include "wpi/hal/DriverStation.h"
#include "wpi/hal/DriverStationTypes.h"
#include "wpi/hal/DutyCycle.h"
#include "wpi/hal/Encoder.h"
// ErrorHandling.hpp
#include "wpi/hal/Errors.h"
#include "wpi/hal/Extensions.h"
#include "wpi/hal/HAL.h"
#include "wpi/hal/I2C.h"
#include "wpi/hal/I2CTypes.h"
#include "wpi/hal/IMU.h"
#include "wpi/hal/IMUTypes.h"
#include "wpi/hal/Main.h"
#include "wpi/hal/Notifier.h"
#include "wpi/hal/Ports.h"
#include "wpi/hal/Power.h"
#include "wpi/hal/PowerDistribution.h"
#include "wpi/hal/PWM.h"
#include "wpi/hal/REVPH.h"
#include "wpi/hal/SerialPort.h"
#include "wpi/hal/SimDevice.h"

// this SERIOUSLY needs ntcore. We're just gonna export it as `unsigned int` instead.
// #include "wpi/hal/SystemServer.h"
#include "wpi/hal/Threads.h"
#include "wpi/hal/Types.h"
#include "wpi/hal/UsageReporting.h"
#include "wpi/hal/Value.h"

// Some classes use WPI_String, so we include this.
// The proper wpihal library will wrap this in safe abstractions that automatically Drop
#include "wpi/util/string.h"

// simulation
#include "wpi/hal/simulation/AddressableLEDData.h"
#include "wpi/hal/simulation/AlertData.h"
#include "wpi/hal/simulation/AnalogInData.h"
#include "wpi/hal/simulation/CanData.h"
#include "wpi/hal/simulation/CTREPCMData.h"
#include "wpi/hal/simulation/DigitalPWMData.h"
#include "wpi/hal/simulation/DIOData.h"
#include "wpi/hal/simulation/DriverStationData.h"
#include "wpi/hal/simulation/DutyCycleData.h"
#include "wpi/hal/simulation/EncoderData.h"
#include "wpi/hal/simulation/I2CData.h"
#include "wpi/hal/simulation/IMUData.h"
#include "wpi/hal/simulation/MockHooks.h"
#include "wpi/hal/simulation/NotifierData.h"
#include "wpi/hal/simulation/NotifyListener.h"
#include "wpi/hal/simulation/PowerDistributionData.h"
#include "wpi/hal/simulation/PWMData.h"
#include "wpi/hal/simulation/Reset.h"
#include "wpi/hal/simulation/REVPHData.h"
#include "wpi/hal/simulation/RoboRioData.h"
//#include "wpi/hal/simulation/SimCallbackRegistry.h"
//#include "wpi/hal/simulation/SimDataValue.h"
#include "wpi/hal/simulation/SimDeviceData.h"

#ifdef __cplusplus
extern "C" {
#endif
// This is a really stupid hack but I don't really want to pull in all of ntcore just for this one fn.
unsigned int HAL_GetSystemServerHandle(void);

#ifdef __cplusplus
}
#endif