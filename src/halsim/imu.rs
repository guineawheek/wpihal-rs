macro_rules! imu_def {
    ($($name:ident),+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct IMU;
        paste::paste! {
            impl IMU {
                $(
                    pub fn [< set_ $name:snake >](value: f64) {
                        unsafe {
                            wpihal_sys::[< HALSIM_SetIMU $name:camel >](value);
                        }
                    }
                )+
            }
        }
    };
}

imu_def!(
    AngleX, AngleY, AngleZ, GyroRateX, GyroRateY, GyroRateZ, AccelX, AccelY, AccelZ, Yaw
);
