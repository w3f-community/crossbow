#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplePermission {
    /// Your app adds photos to the user's photo library.
    NSPhotoLibraryAddUsageDescription,
    /// Your app accesses the user's photo library.
    NSPhotoLibraryUsageDescription,
    /// Your app uses the device camera.
    NSCameraUsageDescription,
    /// Your app uses location services all the time.
    NSLocationAlwaysUsageDescription,
    /// Your app uses location services only when the app is running.
    NSLocationWhenInUseUsageDescription,
    /// DEPRECATED: Update to one of the above instead.
    NSLocationUsageDescription,
    /// Your app uses the address book.
    NSContactsUsageDescription,
    /// Your app uses or modifies the user's calendar information.
    NSCalendarsUsageDescription,
    /// Your app creates reminders in the Reminders app.
    NSRemindersUsageDescription,
    /// Your app uses data from the Health app.
    NSHealthShareUsageDescription,
    /// Your app provides health information to the Health app.
    NSHealthUpdateUsageDescription,
    /// Your app uses the NFC reader.
    NFCReaderUsageDescription,
    /// Your app works with Bluetooth devices.
    NSBluetoothPeripheralUsageDescription,
    /// Your app uses the device microphone.
    NSMicrophoneUsageDescription,
    /// Your app provides a SiriKit Intent.
    NSSiriUsageDescription,
    /// Your app uses speech recognition.
    NSSpeechRecognitionUsageDescription,
    /// Your app uses the device motion tracking hardware.
    NSMotionUsageDescription,
    /// (tvOS only) your app uses the video subscriber account.
    NSVideoSubscriberAccountUsageDescription,
    /// Your app uses Apple Music integration.
    NSAppleMusicUsageDescription,
    /// Your app uses FaceID.
    NSFaceIDUsageDescription,
}

impl std::fmt::Display for ApplePermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NSPhotoLibraryAddUsageDescription => write!(f, "NSPhotoLibraryAddUsageDescription"),
            Self::NSPhotoLibraryUsageDescription => write!(f,"NSPhotoLibraryUsageDescription"),
            Self::NSCameraUsageDescription => write!(f,"NSCameraUsageDescription"),
            Self::NSLocationAlwaysUsageDescription => write!(f, "NSLocationAlwaysUsageDescription"),
            Self::NSLocationWhenInUseUsageDescription => write!(f,"NSLocationWhenInUseUsageDescription"),
            Self::NSLocationUsageDescription => write!(f,"NSLocationUsageDescription"),
            Self::NSContactsUsageDescription => write!(f,"NSContactsUsageDescription"),
            Self::NSCalendarsUsageDescription => write!(f,"NSCalendarsUsageDescription"),
            Self::NSRemindersUsageDescription => write!(f,"NSRemindersUsageDescription"),
            Self::NSHealthShareUsageDescription => write!(f,"NSHealthShareUsageDescription"),
            Self::NSHealthUpdateUsageDescription => write!(f,"NSHealthUpdateUsageDescription"),
            Self::NFCReaderUsageDescription => write!(f,"NFCReaderUsageDescription"),
            Self::NSBluetoothPeripheralUsageDescription => write!(f, "NSBluetoothPeripheralUsageDescription"),
            Self::NSMicrophoneUsageDescription => write!(f,"NSMicrophoneUsageDescription"),
            Self::NSSiriUsageDescription => write!(f,"NSSiriUsageDescription"),
            Self::NSSpeechRecognitionUsageDescription => write!(f,"NSSpeechRecognitionUsageDescription"),
            Self::NSMotionUsageDescription => write!(f,"NSMotionUsageDescription"),
            Self::NSVideoSubscriberAccountUsageDescription => write!(f,"NSVideoSubscriberAccountUsageDescription"),
            Self::NSAppleMusicUsageDescription => write!(f,"NSAppleMusicUsageDescription"),
            Self::NSFaceIDUsageDescription => write!(f,"NSFaceIDUsageDescription")
        }
    }
}