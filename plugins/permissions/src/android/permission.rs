#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPermission {
    /// Allows a calling app to continue a call which was started in another app.
    ///
    /// Protection level: dangerous
    AcceptHandover,
    /// Allows an app to access location in the background.
    ///
    /// Protection level: dangerous
    AccessBackgroundLocation,
    /// Allows an application to access data blobs across users.
    AccessBlobsAcrossUsers,
    /// Allows read/write access to the "properties" table in the checkin database,
    /// to change values that get uploaded.
    ///
    /// Not for use by third-party applications.
    AccessCheckinProperties,
    /// Allows an app to access approximate location.
    ///
    /// Protection level: dangerous
    AccessCoarseLocation,
    /// Allows an app to access precise location.
    ///
    /// Protection level: dangerous
    AccessFineLocation,
    /// Allows an application to access extra location provider commands.
    ///
    /// Protection level: normal
    AccessLocationExtraCommands,
    /// Allows an application to access any geographic locations persisted in the
    /// user's shared collection.
    ///
    /// Protection level: dangerous
    AccessMediaLocation,
    /// Allows applications to access information about networks.
    ///
    /// Protection level: normal
    AccessNetworkState,
    /// Marker permission for applications that wish to access notification policy.
    ///
    /// Protection level: normal
    AccessNotificationPolicy,
    /// Allows applications to access information about Wi-Fi networks.
    ///
    /// Protection level: normal
    AccessWifiState,
    /// Allows applications to call into AccountAuthenticators.
    /// Not for use by third-party applications.
    AccountManager,
    /// Allows an application to recognize physical activity.
    ///
    /// Protection level: dangerous
    ActivityRecognition,
    /// Allows an application to add voicemails into the system.
    ///
    /// Protection level: dangerous
    AddVoicemail,
    /// Allows the app to answer an incoming phone call.
    ///
    /// Protection level: dangerous
    AnswerPhoneCalls,
    /// Allows an application to collect battery statistics.
    ///
    /// Protection level: signature|privileged|development
    BattertStats,
    /// Must be required by an [`AccessibilityService`], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [AccessibilityService]::(https://developer.android.com/reference/android/accessibilityservice/AccessibilityService)
    BindAccessibilityService,
    /// Allows an application to tell the AppWidget service which application can
    /// access AppWidget's data. The normal user flow is that a user picks an
    /// AppWidget to go into a particular host, thereby giving that host
    /// application access to the private data from the AppWidget app. An
    /// application that has this permission should honor that contract.
    ///
    /// Not for use by third-party applications.
    BindAppwidget,
    /// Must be required by a [`AutofillService`], to ensure that only the system
    /// can bind to it.
    ///
    /// Protection level: signature
    ///
    /// [AutofillService]::(https://developer.android.com/reference/android/service/autofill/AutofillService)
    BindAutofillService,
    /// Must be required by a [`CallRedirectionService`], to ensure that only the
    /// system can bind to it.
    ///
    /// Protection level: signature|privileged
    ///
    /// [CallRedirectionService]::(https://developer.android.com/reference/android/telecom/CallRedirectionService)
    BindCallRedirectionService,
    /// A subclass of [`CarrierMessagingClientService`] must be protected with
    /// this permission.
    ///
    /// Protection level: signature
    ///
    /// [CarrierMessagingClientService]::(https://developer.android.com/reference/android/service/carrier/CarrierMessagingClientService)
    BindCarrierMessagingClientService,
    /// This constant was deprecated in API level 23. Use [`BIND_CARRIER_SERVICES`]
    /// instead
    ///
    /// [BIND_CARRIER_SERVICES]::(https://developer.android.com/reference/android/Manifest.permission#BIND_CARRIER_SERVICES)
    BindCarrierMessagingService,
    /// The system process that is allowed to bind to services in carrier apps
    /// will have this permission
    BindCarrierServices,
    /// This constant was deprecated in API level 30. For publishing direct share
    /// targets, please follow the instructions in
    /// https://developer.android.com/training/sharing/receive.html#providing-direct-share-targets instead
    BindChooserTargetService,
    /// Must be required by any [`CompanionDeviceServices`] to ensure that only the
    /// system can bind to it
    ///
    /// [CompanionDeviceServices]::(https://developer.android.com/reference/android/companion/CompanionDeviceService)
    BindCompanionDeviceService,
    /// Must be required by a [`ConditionProviderService`], to ensure that only the
    /// system can bind to it
    ///
    /// [ConditionProviderService]::(https://developer.android.com/reference/android/service/notification/ConditionProviderServic)
    BindConditionProviderService,
    /// Allows SystemUI to request third party controls
    BindControls,
    /// Must be required by device administration receiver, to ensure that only the
    /// system can interact with it
    BindDeviceAdmin,
    /// Must be required by an [`DreamService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [DreamService]::(https://developer.android.com/reference/android/service/dreams/DreamService)
    BindDreamService,
    /// Must be required by a [`InCallService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [InCallService]::(https://developer.android.com/reference/android/telecom/InCallService)
    BindIncallService,
    /// Must be required by an [`InputMethodService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [InputMethodService]::(https://developer.android.com/reference/android/inputmethodservice/InputMethodService)
    BindInputMethod,
    /// Must be required by an [`MidiDeviceService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [MidiDeviceService]::(https://developer.android.com/reference/android/media/midi/MidiDeviceService)
    BindMidiDeviceService,
    /// Must be required by a [`HostApduService`] or [`OffHostApduService`] to ensure
    /// that only the system can bind to it.
    ///
    /// [HostApduService]::(https://developer.android.com/reference/android/nfc/cardemulation/HostApduService)
    /// [OffHostApduService]::(https://developer.android.com/reference/android/nfc/cardemulation/OffHostApduService)
    BindNfcService,
    /// Must be required by an [`NotificationListenerService`], to ensure that only
    /// the system can bind to it.
    ///
    /// [NotificationListenerService]::(https://developer.android.com/reference/android/service/notification/NotificationListenerService)
    BindNotificationListenerService,
    /// Must be required by a [`PrintService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [PrintService]::(https://developer.android.com/reference/android/printservice/PrintService)
    BindPrintService,
    /// Must be required by a [`QuickAccessWalletService`] to ensure that only the
    /// system can bind to it.
    ///
    /// [QuickAccessWalletService]::(https://developer.android.com/reference/android/service/quickaccesswallet/QuickAccessWalletService)
    BindQuickAccessWalletService,
    /// Allows an application to bind to third party quick settings tiles.
    BindQuickSettingsTile,
    /// Must be required by a [`RemoteViewsService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [RemoteViewsService]::(https://developer.android.com/reference/android/widget/RemoteViewsService)
    BindRrmoteviews,
    /// Must be required by a [`CallScreeningService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [CallScreeningService]::(https://developer.android.com/reference/android/telecom/CallScreeningService)
    BindScreeningService,
    /// Must be required by a [`ConnectionService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [ConnectionService]::(https://developer.android.com/reference/android/telecom/ConnectionService)
    BindTelecomConnectionService,
    /// Must be required by a TextService (e.g. SpellCheckerService) to ensure that
    /// only the system can bind to it.
    BindTextService,
    /// Must be required by a [`TvInputService`] to ensure that only the system can
    /// bind to it.
    ///
    /// [TvInputService]::(https://developer.android.com/reference/android/media/tv/TvInputService)
    BindTvInput,
    /// Must be required by a link [`VisualVoicemailService`] to ensure that only the
    /// system can bind to it.
    ///
    /// [VisualVoicemailService]::(https://developer.android.com/reference/android/telephony/VisualVoicemailService)
    BindVisualVoicemailService,
    /// Must be required by a [`VoiceInteractionService`], to ensure that only the
    /// system can bind to it.
    ///
    /// [VoiceInteractionService]::(https://developer.android.com/reference/android/service/voice/VoiceInteractionService)
    BindVoiceInteraction,
    /// Must be required by a [`VpnService`], to ensure that only the system can bind
    /// to it.
    ///
    /// [VpnService]::(https://developer.android.com/reference/android/net/VpnService)
    BindVpnService,
    /// Must be required by an [`VrListenerService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [VrListenerService]::(https://developer.android.com/reference/android/service/vr/VrListenerService)
    BindVrListenerService,
    /// Must be required by a [`WallpaperService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [WallpaperService]::(https://developer.android.com/reference/android/service/wallpaper/WallpaperService)
    BindWallpaper,
    /// Allows applications to connect to paired bluetooth devices
    Bluetooth,
    /// Allows applications to discover and pair bluetooth devices
    BluetoothAdmin,
    /// Required to be able to advertise to nearby Bluetooth devices
    BluetoothAdvertise,
    /// Required to be able to connect to paired Bluetooth devices
    BluetoothConnect,
    /// Allows applications to pair bluetooth devices without user interaction, and to
    /// allow or disallow phonebook access or message access
    BluetoothPrivileged,
    /// Required to be able to discover and pair nearby Bluetooth devices
    BluetoothScan,
    /// Allows an application to access data from sensors that the user uses to measure
    /// what is happening inside their body, such as heart rate
    BodySensors,
    /// Allows an application to broadcast a notification that an application package
    /// has been removed
    BroadcastPackageRemoved,
    /// Allows an application to broadcast an SMS receipt notification
    BroadcastSMS,
    /// Allows an application to broadcast sticky intents
    BroadcastSticky,
    /// Allows an application to broadcast a WAP PUSH receipt notification
    BroadcastWapPush,
    /// Allows an app which implements the [`InCallService`] API to be eligible to be
    /// enabled as a calling companion app
    ///
    /// [InCallService]::(https://developer.android.com/reference/android/telecom/InCallService)
    CallCompanionApp,
    /// Allows an application to initiate a phone call without going through the Dialer
    /// user interface for the user to confirm the call
    CallPhone,
    /// Allows an application to call any phone number, including emergency numbers,
    /// without going through the Dialer user interface for the user to confirm the
    /// call being placed
    CallPrivileged,
    /// Required to be able to access the camera device
    Camera,
    /// Allows an application to capture audio output
    CaptureAudioOutput,
    /// Allows an application to change whether an application component
    /// (other than its own) is enabled or not
    ChangeComponentEnabledState,
    /// Allows an application to modify the current configuration, such as locale
    ChangeConfiguration,
    /// Allows applications to change network connectivity state
    ChangeNetworkState,
    /// Allows applications to enter Wi-Fi Multicast mode
    ChangeWifiMulticastState,
    /// Allows applications to change Wi-Fi connectivity state
    ChangeWifiState,
    /// Allows an application to clear the caches of all installed applications on
    /// the device
    ClearAppCache,
    /// Allows enabling/disabling location update notifications from the radio
    ControlLocationUpdates,
    /// Old permission for deleting an app's cache files, no longer used, but signals
    /// for us to quietly ignore calls instead of throwing an exception.
    DeleteCacheFiles,
    /// Allows an application to delete packages
    DeletePackages,
    /// Allows applications to RW to diagnostic resources
    Diagnostic,
    /// Allows applications to disable the keyguard if it is not secure
    DisableKeyguard,
    /// Allows an application to retrieve state dump information from system services
    Dump,
    /// Allows an application to expand or collapse the status bar
    ExpandStatusBar,
    /// Run as a manufacturer test application, running as the root user
    FactoryTest,
    /// Allows a regular application to use [`Service.startForeground`]
    ///
    /// [Service.startForeground]::(https://developer.android.com/reference/android/app/Service#startForeground(int,%20android.app.Notification))
    ForegroundService,
    /// Allows access to the list of accounts in the Accounts Service
    GetAccounts,
    /// Allows access to the list of accounts in the Accounts Service
    GetAccountsPrivileged,
    /// Allows an application to find out the space used by any package
    GetPackageSize,
    /// This constant was deprecated in API level 21. No longer enforced
    GetTasks,
    /// This permission can be used on content providers to allow the global search
    /// system to access their data
    GlobalSearch,
    /// Allows an app to prevent non-system-overlay windows from being drawn on top
    /// of it
    HighOverlayWindows,
    /// Allows an app to access sensor data with a sampling rate greater than 200 Hz
    HighSamplingRateSensors,
    /// Allows an application to install a location provider into the Location Manager
    InstallLocationProvider,
    /// Allows an application to install packages
    InstallPackages,
    /// Allows an application to install a shortcut in Launcher
    InstallShortcut,
    /// Allows an instant app to create foreground services
    InstantAppForegroundService,
    /// Allows interaction across profiles in the same profile group
    InteractAcrossProfiles,
    /// Allows applications to open network sockets
    Internet,
    /// Allows an application to call [`ActivityManager.killBackgroundProcesses(String)`]
    ///
    /// [ActivityManager.killBackgroundProcesses(String)]::(https://developer.android.com/reference/android/app/ActivityManager#killBackgroundProcesses(java.lang.String))
    KillBackgroundProcesses,
    /// An application needs this permission for
    /// [`Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY`] to show its [`Activity`]
    /// embedded in Settings app.
    ///
    /// [Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY]::(https://developer.android.com/reference/android/provider/Settings#ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY)
    /// [Activity]::(https://developer.android.com/reference/android/app/Activity)
    LaunchMultiPaneSettingsDeepLink,
    /// Allows a data loader to read a package's access logs
    LoaderUsageStats,
    /// Allows an application to use location features in hardware, such as the
    /// geofencing api
    LocationHardware,
    /// Allows an application to manage access to documents, usually as part of a
    /// document picker
    ManageDocuments,
    /// Allows an application a broad access to external storage in scoped storage
    ManageExternalStorage,
    /// Allows an application to modify and delete media files on this device or any
    /// connected storage device without user confirmation
    ManageMedia,
    /// Allows to query ongoing call details and manage ongoing calls
    ///
    /// Protection level: signature|appop
    ManageOngoingCalls,
    /// Allows a calling application which manages its own calls through the
    /// self-managed [`ConnectionService`] APIs
    ///
    /// [ConnectionService]::(https://developer.android.com/reference/android/telecom/ConnectionService)
    ManageOwnCalls,
    /// Not for use by third-party applications
    MasterClear,
    /// Allows an application to know what content is playing and control its
    /// playback
    MediaContentControl,
    /// Allows an application to modify global audio settings
    ModifyAudioSettings,
    /// Allows modification of the telephony state - power on, mmi, etc
    ModifyPhoneState,
    /// Allows formatting file systems for removable storage
    MountFormatFilesystems,
    /// Allows mounting and unmounting file systems for removable storage
    MountUnmountFilesystems,
    /// Allows applications to perform I/O operations over NFC
    NFC,
    /// Allows applications to receive NFC preferred payment service information
    NFCPreferredPatmentInfo,
    /// Allows applications to receive NFC transaction events
    NFCTransactionEvent,
    /// Allows an application to collect component usage statistics
    ///
    /// Declaring the permission implies intention to use the API and the user of
    /// the device can grant permission through the Settings application
    PackageUsageStats,
    /// This constant was deprecated in API level 15. This functionality will be
    /// removed in the future; please do not use. Allow an application to make
    /// its activities persistent
    PersistentActivity,
    /// This constant was deprecated in API level 29. Applications should use
    /// [`CallRedirectionService`] instead of the [`Intent.ACTION_NEW_OUTGOING_CALL`]
    /// broadcast.
    ///
    /// [CallRedirectionService]::(https://developer.android.com/reference/android/telecom/CallRedirectionService)
    /// [Intent.ACTION_NEW_OUTGOING_CALL]::(https://developer.android.com/reference/android/content/Intent#ACTION_NEW_OUTGOING_CALL)
    ProcessOutgoingCalls,
    /// Allows query of any normal app on the device, regardless of manifest
    /// declarations
    QueryAllPackages,
    /// Allows an application to read the user's calendar data
    ReadCalendar,
    /// Allows an application to read the user's call log
    ReadCallLog,
    /// Allows an application to read the user's contacts data
    ReadContacts,
    /// Allows an application to read from external storage
    ReadExternalStorag,
    /// This constant was deprecated in API level 16. The API that used this
    /// permission has been removed
    ReadInputState,
    /// Allows an application to read the low-level system log files
    ReadLogs,
    /// Allows read access to the device's phone number(s)
    ReadPhoneNumbers,
    /// Allows read only access to phone state, including the current cellular
    /// network information, the status of any ongoing calls, and a list of
    /// any [`PhoneAccounts`] registered on the device.
    ///
    /// [PhoneAccounts]::(https://developer.android.com/reference/android/telecom/PhoneAccount)
    ReadPhoneState,
    /// Allows read only access to precise phone state
    ReadPrecisePhoneState,
    /// Allows an application to read SMS messages
    ReadSMS,
    /// Allows applications to read the sync settings
    ReadSyncSettings,
    /// Allows applications to read the sync stats
    ReadSyncStats,
    /// Allows an application to read voicemails in the system
    ReadVoicemail,
    /// Required to be able to reboot the device
    Reboot,
    /// Allows an application to receive the [`Intent.ACTION_BOOT_COMPLETED`]
    /// that is broadcast after the system finishes booting.
    ///
    /// [Intent.ACTION_BOOT_COMPLETED]::(https://developer.android.com/reference/android/content/Intent#ACTION_BOOT_COMPLETED)
    ReceiveBootCompleted,
    /// Allows an application to monitor incoming MMS messages
    ReceiveMMS,
    /// Allows an application to receive SMS messages
    ReceiveSMS,
    /// Allows an application to receive WAP push messages
    ReceiveWapPush,
    /// Allows an application to record audio
    RecordAudio,
    /// Allows an application to change the Z-order of tasks
    ReorderTasks,
    /// Allows app to request to be associated with a device via
    /// CompanionDeviceManager as a "watch"
    ///
    /// Protection level: normal
    RequestCompanionProfileWatch,
    /// Allows a companion app to run in the background
    RequestCompanionRunInBackground,
    /// Allows a companion app to start a foreground service from the background
    RequestCompanionStartForegroundServicesFromBackground,
    /// Allows a companion app to use data in the background
    RequestCompanionUseDataInBackground,
    /// Allows an application to request deleting packages
    RequestDeletePackages,
    /// Permission an application must hold in order to use
    /// [`Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`]
    ///
    /// [Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS]::(https://developer.android.com/reference/android/provider/Settings#ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS)
    RequestIgnoreBatteryOptimizations,
    /// Allows an application to request installing packages
    RequestInstallPackages,
    /// Allows an application to subscribe to notifications about the presence
    /// status change of their associated companion device
    RequestObserveCompanionDevicePresence,
    /// Allows an application to request the screen lock complexity and prompt
    /// users to update the screen lock to a certain complexity level
    RequestPasswordComplexity,
    /// This constant was deprecated in API level 15
    ///
    /// The [`ActivityManager.restartPackage(String)`] API is no longer supported.
    ///
    /// [ActivityManager.restartPackage(String)]::(https://developer.android.com/reference/android/app/ActivityManager#restartPackage(java.lang.String))
    RestartPackages,
    /// Allows applications to use exact alarm APIs
    ScheduleExactAlarm,
    /// Allows an application (Phone) to send a request to other applications to
    /// handle the respond-via-message action during incoming calls.
    SendRespondViaMessage,
    /// Allows an application to send SMS messages
    SendSMS,
    /// Allows an application to broadcast an Intent to set an alarm for the user
    SetAlarm,
    /// Allows an application to control whether activities are immediately finished
    /// when put in the background
    SetAlwaysFinish,
    /// Modify the global animation scaling factor
    SetAnimationScale,
    /// Configure an application for debugging
    SetDebugApp,
    /// This constant was deprecated in API level 15. No longer useful, see
    /// [`PackageManager.addPackageToPreferred(String)`] for details.
    ///
    /// [PackageManager.addPackageToPreferred(String)]::(https://developer.android.com/reference/android/content/pm/PackageManager#addPackageToPreferred(java.lang.String))
    SetPreferredApplications,
    /// Allows an application to set the maximum number of (not needed)
    /// application processes that can be running.
    SetProcessLimit,
    /// Allows applications to set the system time directly
    SetTime,
    /// Allows applications to set the system time zone directly
    SetTimeZone,
    /// Allows applications to set the wallpaper
    SetWallpaper,
    /// Allows applications to set the wallpaper hints
    SetWallpaperHints,
    /// Allow an application to request that a signal be sent to all persistent processes
    SignalPersisteneProcesses,
    /// This constant was deprecated in API level 31. The API that used this permission
    /// is no longer functional
    SMSFinancialTransactions,
    /// Allows an application to start foreground services from the background at any time
    StartForegroundServicesFromBackground,
    /// Allows the holder to start the permission usage screen for an app
    StartViewPermissionUsage,
    /// Allows an application to open, close, or disable the status bar and its icons
    StatusBar,
    /// Allows an app to create windows using the type
    /// [`WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY`], shown on top of all
    /// other apps.
    ///
    /// [WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY]::(https://developer.android.com/reference/android/view/WindowManager.LayoutParams#TYPE_APPLICATION_OVERLAY)
    SystemAlertWindow,
    /// Allows using the device's IR transmitter, if available
    TransmitIr,
    /// Don't use this permission in your app
    UninstallShotycut,
    /// Allows an application to update device statistics
    UpdateDeviceStats,
    /// Allows an application to indicate via
    /// [`PackageInstaller.SessionParams.setRequireUserAction(int)`] that user action
    /// should not be required for an app update
    ///
    /// [PackageInstaller.SessionParams.setRequireUserAction(int)]::(https://developer.android.com/reference/android/content/pm/PackageInstaller.SessionParams#setRequireUserAction(int))
    UpdatePackagesWithoutUserAction,
    /// Allows an app to use device supported biometric modalities
    UseBiometric,
    /// This constant was deprecated in API level 28. Applications should request
    /// [`USE_BIOMETRIC`] instead
    ///
    /// [USE_BIOMETRIC]::(https://developer.android.com/reference/android/Manifest.permission#USE_BIOMETRIC)
    UseFingerprint,
    /// Required for apps targeting [`Build.VERSION_CODES.Q`] that want to use
    /// [`notification full screen intents`]
    ///
    /// [Build.VERSION_CODES.Q]::(https://developer.android.com/reference/android/os/Build.VERSION_CODES#Q)
    /// [notification full screen intents]::(https://developer.android.com/reference/android/app/Notification.Builder#setFullScreenIntent(android.app.PendingIntent,%20boolean))
    UseFullScreenIntent,
    /// Allows to read device identifiers and use ICC based authentication like EAP-AKA
    UseIccAuthWithDeviceIdentifier,
    /// Allows an application to use SIP service
    UseSip,
    /// Required to be able to range to devices using ultra-wideband
    UwbRanging,
    /// Allows access to the vibrator
    Vibrate,
    /// Allows using PowerManager WakeLocks to keep processor from sleeping or screen
    /// from dimming
    WakeLock,
    /// Allows applications to write the apn settings and read sensitive fields of an
    /// existing apn settings like user and password
    WriteApnSettings,
    /// Allows an application to write the user's calendar data
    WriteCalendar,
    /// Allows an application to write (but not read) the user's call log data
    WriteCallLog,
    /// Allows an application to write the user's contacts data
    WriteContacts,
    /// Allows an application to write to external storage
    WriteExternalStorage,
    /// Allows an application to modify the Google service map
    WriteGservices,
    /// Allows an application to read or write the secure system settings
    WriteSecureSettings,
    /// Allows an application to read or write the system settings
    WriteSettings,
    /// Allows applications to write the sync settings
    WriteSyncSettings,
    /// Allows an application to modify and remove existing voicemails in the system
    WriteVoicemail,
}

impl AndroidPermission {
    pub fn get_full_permission(&self) -> String {
        "android.permission.".to_string() + self.to_string().as_str()
    }
}

impl std::fmt::Display for AndroidPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::AcceptHandover => write!(f, "ACCEPT_HANDOVER"),
            Self::AccessBackgroundLocation => write!(f, "ACCESS_BACKGROUND_LOCATION"),
            Self::AccessBlobsAcrossUsers => {
                write!(f, "ACCESS_BLOBS_ACROSS_USERS")
            }
            Self::AccessCheckinProperties => {
                write!(f, "ACCESS_CHECKIN_PROPERTIES")
            }
            Self::AccessCoarseLocation => write!(f, "ACCESS_COARSE_LOCATION"),
            Self::AccessFineLocation => write!(f, "ACCESS_FINE_LOCATION"),
            Self::AccessLocationExtraCommands => {
                write!(f, "ACCESS_LOCATION_EXTRA_COMMANDS")
            }
            Self::AccessMediaLocation => write!(f, "ACCESS_MEDIA_LOCATION"),
            Self::AccessNetworkState => write!(f, "ACCESS_NETWORK_STATE"),
            Self::AccessNotificationPolicy => {
                write!(f, "ACCESS_NOTIFICATION_POLICY")
            }
            Self::AccessWifiState => write!(f, "ACCESS_WIFI_STATE"),
            Self::AccountManager => write!(f, "ACCOUNT_MANAGER"),
            Self::ActivityRecognition => write!(f, "ACTIVITY_RECOGNITION"),
            Self::AddVoicemail => write!(f, "ADD_VOICEMAIL"),
            Self::AnswerPhoneCalls => write!(f, "ANSWER_PHONE_CALLS"),
            Self::BattertStats => write!(f, "BATTERY_STATS"),
            Self::BindAccessibilityService => {
                write!(f, "BIND_ACCESSIBILITY_SERVICE")
            }
            Self::BindAppwidget => write!(f, "BIND_APPWIDGET"),
            Self::BindAutofillService => write!(f, "BIND_AUTOFILL_SERVICE"),
            Self::BindCallRedirectionService => {
                write!(f, "BIND_CALL_REDIRECTION_SERVICE")
            }
            Self::BindCarrierMessagingClientService => {
                write!(f, "BIND_CARRIER_MESSAGING_CLIENT_SERVICE")
            }
            Self::BindCarrierMessagingService => {
                write!(f, "BIND_CARRIER_MESSAGING_SERVICE")
            }
            Self::BindCarrierServices => write!(f, "BIND_CARRIER_SERVICES"),
            Self::BindChooserTargetService => {
                write!(f, "BIND_CHOOSER_TARGET_SERVICE")
            }
            Self::BindCompanionDeviceService => {
                write!(f, "BIND_COMPANION_DEVICE_SERVICE")
            }
            Self::BindConditionProviderService => {
                write!(f, "BIND_CONDITION_PROVIDER_SERVICE")
            }
            Self::BindControls => write!(f, "BIND_CONTROLS"),
            Self::BindDeviceAdmin => write!(f, "BIND_DEVICE_ADMIN"),
            Self::BindDreamService => write!(f, "BIND_DREAM_SERVICE"),
            Self::BindIncallService => write!(f, "BIND_INCALL_SERVICE"),
            Self::BindInputMethod => write!(f, "BIND_INPUT_METHOD"),
            Self::BindMidiDeviceService => {
                write!(f, "BIND_MIDI_DEVICE_SERVICE")
            }
            Self::BindNfcService => write!(f, "BIND_NFC_SERVICE"),
            Self::BindNotificationListenerService => {
                write!(f, "BIND_NOTIFICATION_LISTENER_SERVICE")
            }
            Self::BindPrintService => write!(f, "BIND_PRINT_SERVICE"),
            Self::BindQuickAccessWalletService => {
                write!(f, "BIND_QUICK_ACCESS_WALLET_SERVICE")
            }
            Self::BindQuickSettingsTile => {
                write!(f, "BIND_QUICK_SETTINGS_TILE")
            }
            Self::BindRrmoteviews => write!(f, "BIND_REMOTEVIEWS"),
            Self::BindScreeningService => write!(f, "BIND_SCREENING_SERVICE"),
            Self::BindTelecomConnectionService => {
                write!(f, "BIND_TELECOM_CONNECTION_SERVICE")
            }
            Self::BindTextService => write!(f, "BIND_TEXT_SERVICE"),
            Self::BindTvInput => write!(f, "BIND_TV_INPUT"),
            Self::BindVisualVoicemailService => {
                write!(f, "BIND_VISUAL_VOICEMAIL_SERVICE")
            }
            Self::BindVoiceInteraction => write!(f, "BIND_VOICE_INTERACTION"),
            Self::BindVpnService => write!(f, "BIND_VPN_SERVICE"),
            Self::BindVrListenerService => {
                write!(f, "BIND_VR_LISTENER_SERVICE")
            }
            Self::BindWallpaper => write!(f, "BIND_WALLPAPER"),
            Self::Bluetooth => write!(f, "BLUETOOTH"),
            Self::BluetoothAdmin => write!(f, "BLUETOOTH_ADMIN"),
            Self::BluetoothAdvertise => write!(f, "BLUETOOTH_ADVERTISE"),
            Self::BluetoothConnect => write!(f, "BLUETOOTH_CONNECT"),
            Self::BluetoothPrivileged => write!(f, "BLUETOOTH_PRIVILEGED"),
            Self::BluetoothScan => write!(f, "BLUETOOTH_SCAN"),
            Self::BodySensors => write!(f, "BODY_SENSORS"),
            Self::BroadcastPackageRemoved => {
                write!(f, "BROADCAST_PACKAGE_REMOVED")
            }
            Self::BroadcastSMS => write!(f, "BROADCAST_SMS"),
            Self::BroadcastSticky => write!(f, "BROADCAST_STICKY"),
            Self::BroadcastWapPush => write!(f, "BROADCAST_WAP_PUSH"),
            Self::CallCompanionApp => write!(f, "CALL_COMPANION_APP"),
            Self::CallPhone => write!(f, "CALL_PHONE"),
            Self::CallPrivileged => write!(f, "CALL_PRIVILEGED"),
            Self::Camera => write!(f, "CAMERA"),
            Self::CaptureAudioOutput => write!(f, "CAPTURE_AUDIO_OUTPUT"),
            Self::ChangeComponentEnabledState => {
                write!(f, "CHANGE_COMPONENT_ENABLED_STATE")
            }
            Self::ChangeConfiguration => write!(f, "CHANGE_CONFIGURATION"),
            Self::ChangeNetworkState => write!(f, "CHANGE_NETWORK_STATE"),
            Self::ChangeWifiMulticastState => {
                write!(f, "CHANGE_WIFI_MULTICAST_STATE")
            }
            Self::ChangeWifiState => write!(f, "CHANGE_WIFI_STATE"),
            Self::ClearAppCache => write!(f, "CLEAR_APP_CACHE"),
            Self::ControlLocationUpdates => {
                write!(f, "CONTROL_LOCATION_UPDATES")
            }
            Self::DeleteCacheFiles => write!(f, "DELETE_CACHE_FILES"),
            Self::DeletePackages => write!(f, "DELETE_PACKAGES"),
            Self::Diagnostic => write!(f, "DIAGNOSTIC"),
            Self::DisableKeyguard => write!(f, "DISABLE_KEYGUARD"),
            Self::Dump => write!(f, "DUMP"),
            Self::ExpandStatusBar => write!(f, "EXPAND_STATUS_BAR"),
            Self::FactoryTest => write!(f, "FACTORY_TEST"),
            Self::ForegroundService => write!(f, "FOREGROUND_SERVICE"),
            Self::GetAccounts => write!(f, "GET_ACCOUNTS"),
            Self::GetAccountsPrivileged => {
                write!(f, "GET_ACCOUNTS_PRIVILEGED")
            }
            Self::GetPackageSize => write!(f, "GET_PACKAGE_SIZE"),
            Self::GetTasks => write!(f, "GET_TASKS"),
            Self::GlobalSearch => write!(f, "GLOBAL_SEARCH"),
            Self::HighOverlayWindows => write!(f, "HIDE_OVERLAY_WINDOWS"),
            Self::HighSamplingRateSensors => {
                write!(f, "HIGH_SAMPLING_RATE_SENSORS")
            }
            Self::InstallLocationProvider => {
                write!(f, "INSTALL_LOCATION_PROVIDER")
            }
            Self::InstallPackages => write!(f, "INSTALL_PACKAGES"),
            Self::InstallShortcut => write!(f, "INSTALL_SHORTCUT"),
            Self::InstantAppForegroundService => {
                write!(f, "INSTANT_APP_FOREGROUND_SERVICE")
            }
            Self::InteractAcrossProfiles => {
                write!(f, "INTERACT_ACROSS_PROFILES")
            }
            Self::Internet => write!(f, "INTERNET"),
            Self::KillBackgroundProcesses => {
                write!(f, "KILL_BACKGROUND_PROCESSES")
            }
            Self::LaunchMultiPaneSettingsDeepLink => {
                write!(f, "LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK")
            }
            Self::LoaderUsageStats => write!(f, "LOADER_USAGE_STATS"),
            Self::LocationHardware => write!(f, "LOCATION_HARDWARE"),
            Self::ManageDocuments => write!(f, "MANAGE_DOCUMENTS"),
            Self::ManageExternalStorage => {
                write!(f, "MANAGE_EXTERNAL_STORAGE")
            }
            Self::ManageMedia => write!(f, "MANAGE_MEDIA"),
            Self::ManageOngoingCalls => write!(f, "MANAGE_ONGOING_CALLS"),
            Self::ManageOwnCalls => write!(f, "MANAGE_OWN_CALLS"),
            Self::MasterClear => write!(f, "MASTER_CLEAR"),
            Self::MediaContentControl => write!(f, "MEDIA_CONTENT_CONTROL"),
            Self::ModifyAudioSettings => write!(f, "MODIFY_AUDIO_SETTINGS"),
            Self::ModifyPhoneState => write!(f, "MODIFY_PHONE_STATE"),
            Self::MountFormatFilesystems => {
                write!(f, "MOUNT_FORMAT_FILESYSTEMS")
            }
            Self::MountUnmountFilesystems => {
                write!(f, "MOUNT_UNMOUNT_FILESYSTEMS")
            }
            Self::NFC => write!(f, "NFC"),
            Self::NFCPreferredPatmentInfo => {
                write!(f, "NFC_PREFERRED_PAYMENT_INFO")
            }
            Self::NFCTransactionEvent => write!(f, "NFC_TRANSACTION_EVENT"),
            Self::PackageUsageStats => write!(f, "PACKAGE_USAGE_STATS"),
            Self::PersistentActivity => write!(f, "PERSISTENT_ACTIVITY"),
            Self::ProcessOutgoingCalls => write!(f, "PROCESS_OUTGOING_CALLS"),
            Self::QueryAllPackages => write!(f, "QUERY_ALL_PACKAGES"),
            Self::ReadCalendar => write!(f, "READ_CALENDAR"),
            Self::ReadCallLog => write!(f, "READ_CALL_LOG"),
            Self::ReadContacts => write!(f, "READ_CONTACTS"),
            Self::ReadExternalStorag => write!(f, "READ_EXTERNAL_STORAG"),
            Self::ReadInputState => write!(f, "READ_INPUT_STATE"),
            Self::ReadLogs => write!(f, "READ_LOGS"),
            Self::ReadPhoneNumbers => write!(f, "READ_PHONE_NUMBERS"),
            Self::ReadPhoneState => write!(f, "READ_PHONE_STATE"),
            Self::ReadPrecisePhoneState => {
                write!(f, "READ_PRECISE_PHONE_STATE")
            }
            Self::ReadSMS => write!(f, "READ_SMS"),
            Self::ReadSyncSettings => write!(f, "READ_SYNC_SETTINGS"),
            Self::ReadSyncStats => write!(f, "READ_SYNC_STATS"),
            Self::ReadVoicemail => write!(f, "READ_VOICEMAIL"),
            Self::Reboot => write!(f, "REBOOT"),
            Self::ReceiveBootCompleted => write!(f, "RECEIVE_BOOT_COMPLETED"),
            Self::ReceiveMMS => write!(f, "RECEIVE_MMS"),
            Self::ReceiveSMS => write!(f, "RECEIVE_SMS"),
            Self::ReceiveWapPush => write!(f, "RECEIVE_WAP_PUSH"),
            Self::RecordAudio => write!(f, "RECORD_AUDIO"),
            Self::ReorderTasks => write!(f, "REORDER_TASKS"),
            Self::RequestCompanionProfileWatch => write!(f, "REQUEST_COMPANION_PROFILE_WATCH"),
            Self::RequestCompanionRunInBackground => {
                write!(f, "REQUEST_COMPANION_RUN_IN_BACKGROUND")
            }
            Self::RequestCompanionStartForegroundServicesFromBackground => write!(
                f,
                "REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND"
            ),
            Self::RequestCompanionUseDataInBackground => {
                write!(f, "REQUEST_COMPANION_USE_DATA_IN_BACKGROUND")
            }
            Self::RequestDeletePackages => {
                write!(f, "REQUEST_DELETE_PACKAGES")
            }
            Self::RequestIgnoreBatteryOptimizations => {
                write!(f, "REQUEST_IGNORE_BATTERY_OPTIMIZATIONS")
            }
            Self::RequestInstallPackages => {
                write!(f, "REQUEST_INSTALL_PACKAGES")
            }
            Self::RequestObserveCompanionDevicePresence => {
                write!(f, "REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE")
            }
            Self::RequestPasswordComplexity => {
                write!(f, "REQUEST_PASSWORD_COMPLEXITY")
            }
            Self::RestartPackages => write!(f, "RESTART_PACKAGES"),
            Self::ScheduleExactAlarm => write!(f, "SCHEDULE_EXACT_ALARM"),
            Self::SendRespondViaMessage => {
                write!(f, "SEND_RESPOND_VIA_MESSAGE")
            }
            Self::SendSMS => write!(f, "SEND_SMS"),
            Self::SetAlarm => write!(f, "SET_ALARM"),
            Self::SetAlwaysFinish => write!(f, "SET_ALWAYS_FINISH"),
            Self::SetAnimationScale => write!(f, "SET_ANIMATION_SCALE"),
            Self::SetDebugApp => write!(f, "SET_DEBUG_APP"),
            Self::SetPreferredApplications => {
                write!(f, "SET_PREFERRED_APPLICATIONS")
            }
            Self::SetProcessLimit => write!(f, "SET_PROCESS_LIMIT"),
            Self::SetTime => write!(f, "SET_TIME"),
            Self::SetTimeZone => write!(f, "SET_TIME_ZONE"),
            Self::SetWallpaper => write!(f, "SET_WALLPAPER"),
            Self::SetWallpaperHints => write!(f, "SET_WALLPAPER_HINTS"),
            Self::SignalPersisteneProcesses => {
                write!(f, "SIGNAL_PERSISTENT_PROCESSES")
            }
            Self::SMSFinancialTransactions => {
                write!(f, "SMS_FINANCIAL_TRANSACTIONS")
            }
            Self::StartForegroundServicesFromBackground => {
                write!(f, "START_FOREGROUND_SERVICES_FROM_BACKGROUND")
            }
            Self::StartViewPermissionUsage => {
                write!(f, "START_VIEW_PERMISSION_USAGE")
            }
            Self::StatusBar => write!(f, "STATUS_BAR"),
            Self::SystemAlertWindow => write!(f, "SYSTEM_ALERT_WINDOW"),
            Self::TransmitIr => write!(f, "TRANSMIT_IR"),
            Self::UninstallShotycut => write!(f, "UNINSTALL_SHORTCUT"),
            Self::UpdateDeviceStats => write!(f, "UPDATE_DEVICE_STATS"),
            Self::UpdatePackagesWithoutUserAction => {
                write!(f, "UPDATE_PACKAGES_WITHOUT_USER_ACTION")
            }
            Self::UseBiometric => write!(f, "USE_BIOMETRIC"),
            Self::UseFingerprint => write!(f, "USE_FINGERPRINT"),
            Self::UseFullScreenIntent => write!(f, "USE_FULL_SCREEN_INTENT"),
            Self::UseIccAuthWithDeviceIdentifier => {
                write!(f, "USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER")
            }
            Self::UseSip => write!(f, "USE_SIP"),
            Self::UwbRanging => write!(f, "UWB_RANGING"),
            Self::Vibrate => write!(f, "VIBRATE"),
            Self::WakeLock => write!(f, "WAKE_LOCK"),
            Self::WriteApnSettings => write!(f, "WRITE_APN_SETTINGS"),
            Self::WriteCalendar => write!(f, "WRITE_CALENDAR"),
            Self::WriteCallLog => write!(f, "WRITE_CALL_LOG"),
            Self::WriteContacts => write!(f, "WRITE_CONTACTS"),
            Self::WriteExternalStorage => write!(f, "WRITE_EXTERNAL_STORAGE"),
            Self::WriteGservices => write!(f, "WRITE_GSERVICES"),
            Self::WriteSecureSettings => write!(f, "WRITE_SECURE_SETTINGS"),
            Self::WriteSettings => write!(f, "WRITE_SETTINGS"),
            Self::WriteSyncSettings => write!(f, "WRITE_SYNC_SETTINGS"),
            Self::WriteVoicemail => write!(f, "WRITE_VOICEMAIL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_permission() {
        let permission = AndroidPermission::AccessCheckinProperties;
        assert_eq!(
            permission.get_full_permission(),
            "android.permission.ACCESS_CHECKIN_PROPERTIES"
        );
    }
}
