#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPermissionGroup {
    /// Used for permissions that are associated with activity recognition
    ACTIVITY_RECOGNITION,
    /// Used for runtime permissions related to user's calendar
    CALENDAR,
    /// Used for permissions that are associated telephony features
    CALL_LOG,
    /// Used for permissions that are associated with accessing camera or
    /// capturing images/video from the device
    CAMERA,
    /// Used for runtime permissions related to contacts and profiles on this
    /// device
    CONTACTS,
    /// Used for permissions that allow accessing the device location
    LOCATION,
    /// Used for permissions that are associated with accessing microphone audio
    /// from the device
    MICROPHONE,
    /// Required to be able to discover and connect to nearby Bluetooth devices
    NEARBY_DEVICES,
    /// Used for permissions that are associated telephony features
    PHONE,
    /// Used for permissions that are associated with accessing body or
    /// environmental sensors
    SENSORS,
    /// Used for runtime permissions related to user's SMS messages
    SMS,
    /// Used for runtime permissions related to the shared external storage
    STORAGE,
}

impl std::fmt::Display for AndroidPermissionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ACTIVITY_RECOGNITION => {
                write!(f, "android.permission-group.ACTIVITY_RECOGNITION")
            }
            Self::CALENDAR => write!(f, "android.permission-group.CALENDAR"),
            Self::CALL_LOG => write!(f, "android.permission-group.CALL_LOG"),
            Self::CAMERA => write!(f, "android.permission-group.CAMERA"),
            Self::CONTACTS => write!(f, "android.permission-group.CONTACTS"),
            Self::LOCATION => write!(f, "android.permission-group.LOCATION"),
            Self::MICROPHONE => write!(f, "android.permission-group.MICROPHONE"),
            Self::NEARBY_DEVICES => write!(f, "android.permission-group.NEARBY_DEVICES"),
            Self::PHONE => write!(f, "android.permission-group.PHONE"),
            Self::SENSORS => write!(f, "android.permission-group.SENSORS"),
            Self::SMS => write!(f, "android.permission-group.SMS"),
            Self::STORAGE => write!(f, "android.permission-group.STORAGE"),
        }
    }
}
