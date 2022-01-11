#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPermission {
    /// Allows a calling app to continue a call which was started in another app.
    ACCEPT_HANDOVER,
    /// Allows an app to access location in the background.
    ACCESS_BACKGROUND_LOCATION,
    /// Allows an application to access data blobs across users.
    ACCESS_BLOBS_ACROSS_USERS,
    /// Allows read/write access to the "properties" table in the checkin database,
    /// to change values that get uploaded.
    ACCESS_CHECKIN_PROPERTIES,
    /// Allows an app to access approximate location.
    ACCESS_COARSE_LOCATION,
    /// Allows an app to access precise location.
    ACCESS_FINE_LOCATION,
    /// Allows an application to access extra location provider commands.
    ACCESS_LOCATION_EXTRA_COMMANDS,
    /// Allows an application to access any geographic locations persisted in the
    /// user's shared collection.
    ACCESS_MEDIA_LOCATION,
    /// Allows applications to access information about networks.
    ACCESS_NETWORK_STATE,
    /// Marker permission for applications that wish to access notification policy.
    ACCESS_NOTIFICATION_POLICY,
    /// Allows applications to access information about Wi-Fi networks.
    ACCESS_WIFI_STATE,
    /// Allows applications to call into AccountAuthenticators.
    ACCOUNT_MANAGER,
    /// Allows an application to recognize physical activity.
    ACTIVITY_RECOGNITION,
    /// Allows an application to add voicemails into the system.
    ADD_VOICEMAIL,
    /// Allows the app to answer an incoming phone call.
    ANSWER_PHONE_CALLS,
    /// Allows an application to collect battery statistics
    ///
    /// Protection level: signature|privileged|development
    BATTERY_STATS,
    /// Must be required by an [`AccessibilityService`], to ensure that only the
    /// system can bind to it
    ///
    /// [AccessibilityService]::(https://developer.android.com/reference/android/accessibilityservice/AccessibilityService)
    BIND_ACCESSIBILITY_SERVICE,
    /// Allows an application to tell the AppWidget service which application can
    /// access AppWidget's data
    BIND_APPWIDGET,
    /// Must be required by a [`AutofillService`], to ensure that only the system
    /// can bind to it
    ///
    /// [AutofillService]::(https://developer.android.com/reference/android/service/autofill/AutofillService)
    BIND_AUTOFILL_SERVICE,
    /// Must be required by a [`CallRedirectionService`], to ensure that only the
    /// system can bind to it
    ///
    /// [CallRedirectionService]::(https://developer.android.com/reference/android/telecom/CallRedirectionService)
    BIND_CALL_REDIRECTION_SERVICE,
    /// A subclass of [`CarrierMessagingClientService`] must be protected with
    /// this permission
    ///
    /// [CarrierMessagingClientService]::(https://developer.android.com/reference/android/service/carrier/CarrierMessagingClientService)
    BIND_CARRIER_MESSAGING_CLIENT_SERVICE,
    /// This constant was deprecated in API level 23. Use [`BIND_CARRIER_SERVICES`]
    /// instead
    ///
    /// [BIND_CARRIER_SERVICES]::(https://developer.android.com/reference/android/Manifest.permission#BIND_CARRIER_SERVICES)
    BIND_CARRIER_MESSAGING_SERVICE,
    /// The system process that is allowed to bind to services in carrier apps
    /// will have this permission
    BIND_CARRIER_SERVICES,
    /// This constant was deprecated in API level 30. For publishing direct share
    /// targets, please follow the instructions in
    /// https://developer.android.com/training/sharing/receive.html#providing-direct-share-targets instead
    BIND_CHOOSER_TARGET_SERVICE,
    /// Must be required by any [`CompanionDeviceServices`] to ensure that only the
    /// system can bind to it
    ///
    /// [CompanionDeviceServices]::(https://developer.android.com/reference/android/companion/CompanionDeviceService)
    BIND_COMPANION_DEVICE_SERVICE,
    /// Must be required by a [`ConditionProviderService`], to ensure that only the
    /// system can bind to it
    ///
    /// [ConditionProviderService]::(https://developer.android.com/reference/android/service/notification/ConditionProviderServic)
    BIND_CONDITION_PROVIDER_SERVICE,
    /// Allows SystemUI to request third party controls
    BIND_CONTROLS,
    /// Must be required by device administration receiver, to ensure that only the
    /// system can interact with it
    BIND_DEVICE_ADMIN,
    /// Must be required by an [`DreamService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [DreamService]::(https://developer.android.com/reference/android/service/dreams/DreamService)
    BIND_DREAM_SERVICE,
    /// Must be required by a [`InCallService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [InCallService]::(https://developer.android.com/reference/android/telecom/InCallService)
    BIND_INCALL_SERVICE,
    /// Must be required by an [`InputMethodService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [InputMethodService]::(https://developer.android.com/reference/android/inputmethodservice/InputMethodService)
    BIND_INPUT_METHOD,
    /// Must be required by an [`MidiDeviceService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [MidiDeviceService]::(https://developer.android.com/reference/android/media/midi/MidiDeviceService)
    BIND_MIDI_DEVICE_SERVICE,
    /// Must be required by a [`HostApduService`] or [`OffHostApduService`] to ensure
    /// that only the system can bind to it.
    ///
    /// [HostApduService]::(https://developer.android.com/reference/android/nfc/cardemulation/HostApduService)
    /// [OffHostApduService]::(https://developer.android.com/reference/android/nfc/cardemulation/OffHostApduService)
    BIND_NFC_SERVICE,
    /// Must be required by an [`NotificationListenerService`], to ensure that only
    /// the system can bind to it.
    ///
    /// [NotificationListenerService]::(https://developer.android.com/reference/android/service/notification/NotificationListenerService)
    BIND_NOTIFICATION_LISTENER_SERVICE,
    /// Must be required by a [`PrintService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [PrintService]::(https://developer.android.com/reference/android/printservice/PrintService)
    BIND_PRINT_SERVICE,
    /// Must be required by a [`QuickAccessWalletService`] to ensure that only the
    /// system can bind to it.
    ///
    /// [QuickAccessWalletService]::(https://developer.android.com/reference/android/service/quickaccesswallet/QuickAccessWalletService)
    BIND_QUICK_ACCESS_WALLET_SERVICE,
    /// Allows an application to bind to third party quick settings tiles.
    BIND_QUICK_SETTINGS_TILE,
    /// Must be required by a [`RemoteViewsService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [RemoteViewsService]::(https://developer.android.com/reference/android/widget/RemoteViewsService)
    BIND_REMOTEVIEWS,
    /// Must be required by a [`CallScreeningService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [CallScreeningService]::(https://developer.android.com/reference/android/telecom/CallScreeningService)
    BIND_SCREENING_SERVICE,
    /// Must be required by a [`ConnectionService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [ConnectionService]::(https://developer.android.com/reference/android/telecom/ConnectionService)
    BIND_TELECOM_CONNECTION_SERVICE,
    /// Must be required by a TextService (e.g. SpellCheckerService) to ensure that
    /// only the system can bind to it.
    BIND_TEXT_SERVICE,
    /// Must be required by a [`TvInputService`] to ensure that only the system can
    /// bind to it.
    ///
    /// [TvInputService]::(https://developer.android.com/reference/android/media/tv/TvInputService)
    BIND_TV_INPUT,
    /// Must be required by a link [`VisualVoicemailService`] to ensure that only the
    /// system can bind to it.
    ///
    /// [VisualVoicemailService]::(https://developer.android.com/reference/android/telephony/VisualVoicemailService)
    BIND_VISUAL_VOICEMAIL_SERVICE,
    /// Must be required by a [`VoiceInteractionService`], to ensure that only the
    /// system can bind to it.
    ///
    /// [VoiceInteractionService]::(https://developer.android.com/reference/android/service/voice/VoiceInteractionService)
    BIND_VOICE_INTERACTION,
    /// Must be required by a [`VpnService`], to ensure that only the system can bind
    /// to it.
    ///
    /// [VpnService]::(https://developer.android.com/reference/android/net/VpnService)
    BIND_VPN_SERVICE,
    /// Must be required by an [`VrListenerService`], to ensure that only the system
    /// can bind to it.
    ///
    /// [VrListenerService]::(https://developer.android.com/reference/android/service/vr/VrListenerService)
    BIND_VR_LISTENER_SERVICE,
    /// Must be required by a [`WallpaperService`], to ensure that only the system can
    /// bind to it.
    ///
    /// [WallpaperService]::(https://developer.android.com/reference/android/service/wallpaper/WallpaperService)
    BIND_WALLPAPER,
    /// Allows applications to connect to paired bluetooth devices
    BLUETOOTH,
    /// Allows applications to discover and pair bluetooth devices
    BLUETOOTH_ADMIN,
    /// Required to be able to advertise to nearby Bluetooth devices
    BLUETOOTH_ADVERTISE,
    /// Required to be able to connect to paired Bluetooth devices
    BLUETOOTH_CONNECT,
    /// Allows applications to pair bluetooth devices without user interaction, and to
    /// allow or disallow phonebook access or message access
    BLUETOOTH_PRIVILEGED,
    /// Required to be able to discover and pair nearby Bluetooth devices
    BLUETOOTH_SCAN,
    /// Allows an application to access data from sensors that the user uses to measure
    /// what is happening inside their body, such as heart rate
    BODY_SENSORS,
    /// Allows an application to broadcast a notification that an application package
    /// has been removed
    BROADCAST_PACKAGE_REMOVED,
    /// Allows an application to broadcast an SMS receipt notification
    BROADCAST_SMS,
    /// Allows an application to broadcast sticky intents
    BROADCAST_STICKY,
    /// Allows an application to broadcast a WAP PUSH receipt notification
    BROADCAST_WAP_PUSH,
    /// Allows an app which implements the [`InCallService`] API to be eligible to be
    /// enabled as a calling companion app
    ///
    /// [InCallService]::(https://developer.android.com/reference/android/telecom/InCallService)
    CALL_COMPANION_APP,
    /// Allows an application to initiate a phone call without going through the Dialer
    /// user interface for the user to confirm the call
    CALL_PHONE,
    /// Allows an application to call any phone number, including emergency numbers,
    /// without going through the Dialer user interface for the user to confirm the
    /// call being placed
    CALL_PRIVILEGED,
    /// Required to be able to access the camera device
    CAMERA,
    /// Allows an application to capture audio output
    CAPTURE_AUDIO_OUTPUT,
    /// Allows an application to change whether an application component
    /// (other than its own) is enabled or not
    CHANGE_COMPONENT_ENABLED_STATE,
    /// Allows an application to modify the current configuration, such as locale
    CHANGE_CONFIGURATION,
    /// Allows applications to change network connectivity state
    CHANGE_NETWORK_STATE,
    /// Allows applications to enter Wi-Fi Multicast mode
    CHANGE_WIFI_MULTICAST_STATE,
    /// Allows applications to change Wi-Fi connectivity state
    CHANGE_WIFI_STATE,
    /// Allows an application to clear the caches of all installed applications on
    /// the device
    CLEAR_APP_CACHE,
    /// Allows enabling/disabling location update notifications from the radio
    CONTROL_LOCATION_UPDATES,
    /// Old permission for deleting an app's cache files, no longer used, but signals
    /// for us to quietly ignore calls instead of throwing an exception.
    DELETE_CACHE_FILES,
    /// Allows an application to delete packages
    DELETE_PACKAGES,
    /// Allows applications to RW to diagnostic resources
    DIAGNOSTIC,
    /// Allows applications to disable the keyguard if it is not secure
    DISABLE_KEYGUARD,
    /// Allows an application to retrieve state dump information from system services
    DUMP,
    /// Allows an application to expand or collapse the status bar
    EXPAND_STATUS_BAR,
    /// Run as a manufacturer test application, running as the root user
    FACTORY_TEST,
    /// Allows a regular application to use [`Service.startForeground`]
    ///
    /// [Service.startForeground]::(https://developer.android.com/reference/android/app/Service#startForeground(int,%20android.app.Notification))
    FOREGROUND_SERVICE,
    /// Allows access to the list of accounts in the Accounts Service
    GET_ACCOUNTS,
    /// Allows access to the list of accounts in the Accounts Service
    GET_ACCOUNTS_PRIVILEGED,
    /// Allows an application to find out the space used by any package
    GET_PACKAGE_SIZE,
    /// This constant was deprecated in API level 21. No longer enforced
    GET_TASKS,
    /// This permission can be used on content providers to allow the global search
    /// system to access their data
    GLOBAL_SEARCH,
    /// Allows an app to prevent non-system-overlay windows from being drawn on top
    /// of it
    HIDE_OVERLAY_WINDOWS,
    /// Allows an app to access sensor data with a sampling rate greater than 200 Hz
    HIGH_SAMPLING_RATE_SENSORS,
    /// Allows an application to install a location provider into the Location Manager
    INSTALL_LOCATION_PROVIDER,
    /// Allows an application to install packages
    INSTALL_PACKAGES,
    /// Allows an application to install a shortcut in Launcher
    INSTALL_SHORTCUT,
    /// Allows an instant app to create foreground services
    INSTANT_APP_FOREGROUND_SERVICE,
    /// Allows interaction across profiles in the same profile group
    INTERACT_ACROSS_PROFILES,
    /// Allows applications to open network sockets
    INTERNET,
    /// Allows an application to call [`ActivityManager.killBackgroundProcesses(String)`]
    ///
    /// [ActivityManager.killBackgroundProcesses(String)]::(https://developer.android.com/reference/android/app/ActivityManager#killBackgroundProcesses(java.lang.String))
    KILL_BACKGROUND_PROCESSES,
    /// An application needs this permission for
    /// [`Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY`] to show its [`Activity`]
    /// embedded in Settings app.
    ///
    /// [Settings.ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY]::(https://developer.android.com/reference/android/provider/Settings#ACTION_SETTINGS_EMBED_DEEP_LINK_ACTIVITY)
    /// [Activity]::(https://developer.android.com/reference/android/app/Activity)
    LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK,
    /// Allows a data loader to read a package's access logs
    LOADER_USAGE_STATS,
    /// Allows an application to use location features in hardware, such as the
    /// geofencing api
    LOCATION_HARDWARE,
    /// Allows an application to manage access to documents, usually as part of a
    /// document picker
    MANAGE_DOCUMENTS,
    /// Allows an application a broad access to external storage in scoped storage
    MANAGE_EXTERNAL_STORAGE,
    /// Allows an application to modify and delete media files on this device or any
    /// connected storage device without user confirmation
    MANAGE_MEDIA,
    /// Allows to query ongoing call details and manage ongoing calls
    ///
    /// Protection level: signature|appop
    MANAGE_ONGOING_CALLS,
    /// Allows a calling application which manages its own calls through the
    /// self-managed [`ConnectionService`] APIs
    ///
    /// [ConnectionService]::(https://developer.android.com/reference/android/telecom/ConnectionService)
    MANAGE_OWN_CALLS,
    /// Not for use by third-party applications
    MASTER_CLEAR,
    /// Allows an application to know what content is playing and control its
    /// playback
    MEDIA_CONTENT_CONTROL,
    /// Allows an application to modify global audio settings
    MODIFY_AUDIO_SETTINGS,
    /// Allows modification of the telephony state - power on, mmi, etc
    MODIFY_PHONE_STATE,
    /// Allows formatting file systems for removable storage
    MOUNT_FORMAT_FILESYSTEMS,
    /// Allows mounting and unmounting file systems for removable storage
    MOUNT_UNMOUNT_FILESYSTEMS,
    /// Allows applications to perform I/O operations over NFC
    NFC,
    /// Allows applications to receive NFC preferred payment service information
    NFC_PREFERRED_PAYMENT_INFO,
    /// Allows applications to receive NFC transaction events
    NFC_TRANSACTION_EVENT,
    /// Allows an application to collect component usage statistics
    ///
    /// Declaring the permission implies intention to use the API and the user of
    /// the device can grant permission through the Settings application
    PACKAGE_USAGE_STATS,
    /// This constant was deprecated in API level 15. This functionality will be
    /// removed in the future; please do not use. Allow an application to make
    /// its activities persistent
    PERSISTENT_ACTIVITY,
    /// This constant was deprecated in API level 29. Applications should use
    /// [`CallRedirectionService`] instead of the [`Intent.ACTION_NEW_OUTGOING_CALL`]
    /// broadcast.
    ///
    /// [CallRedirectionService]::(https://developer.android.com/reference/android/telecom/CallRedirectionService)
    /// [Intent.ACTION_NEW_OUTGOING_CALL]::(https://developer.android.com/reference/android/content/Intent#ACTION_NEW_OUTGOING_CALL)
    PROCESS_OUTGOING_CALLS,
    /// Allows query of any normal app on the device, regardless of manifest
    /// declarations
    QUERY_ALL_PACKAGES,
    /// Allows an application to read the user's calendar data
    READ_CALENDAR,
    /// Allows an application to read the user's call log
    READ_CALL_LOG,
    /// Allows an application to read the user's contacts data
    READ_CONTACTS,
    /// Allows an application to read from external storage
    READ_EXTERNAL_STORAG,
    /// This constant was deprecated in API level 16. The API that used this
    /// permission has been removed
    READ_INPUT_STATE,
    /// Allows an application to read the low-level system log files
    READ_LOGS,
    /// Allows read access to the device's phone number(s)
    READ_PHONE_NUMBERS,
    /// Allows read only access to phone state, including the current cellular
    /// network information, the status of any ongoing calls, and a list of
    /// any [`PhoneAccounts`] registered on the device.
    ///
    /// [PhoneAccounts]::(https://developer.android.com/reference/android/telecom/PhoneAccount)
    READ_PHONE_STATE,
    /// Allows read only access to precise phone state
    READ_PRECISE_PHONE_STATE,
    /// Allows an application to read SMS messages
    READ_SMS,
    /// Allows applications to read the sync settings
    READ_SYNC_SETTINGS,
    /// Allows applications to read the sync stats
    READ_SYNC_STATS,
    /// Allows an application to read voicemails in the system
    READ_VOICEMAIL,
    /// Required to be able to reboot the device
    REBOOT,
    /// Allows an application to receive the [`Intent.ACTION_BOOT_COMPLETED`]
    /// that is broadcast after the system finishes booting.
    ///
    /// [Intent.ACTION_BOOT_COMPLETED]::(https://developer.android.com/reference/android/content/Intent#ACTION_BOOT_COMPLETED)
    RECEIVE_BOOT_COMPLETED,
    /// Allows an application to monitor incoming MMS messages
    RECEIVE_MMS,
    /// Allows an application to receive SMS messages
    RECEIVE_SMS,
    /// Allows an application to receive WAP push messages
    RECEIVE_WAP_PUSH,
    /// Allows an application to record audio
    RECORD_AUDIO,
    /// Allows an application to change the Z-order of tasks
    REORDER_TASKS,
    /// Allows app to request to be associated with a device via
    /// CompanionDeviceManager as a "watch"
    ///
    /// Protection level: normal
    REQUEST_COMPANION_PROFILE_WATCH,
    /// Allows a companion app to run in the background
    REQUEST_COMPANION_RUN_IN_BACKGROUND,
    /// Allows a companion app to start a foreground service from the background
    REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND,
    /// Allows a companion app to use data in the background
    REQUEST_COMPANION_USE_DATA_IN_BACKGROUND,
    /// Allows an application to request deleting packages
    REQUEST_DELETE_PACKAGES,
    /// Permission an application must hold in order to use
    /// [`Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS`]
    ///
    /// [Settings.ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS]::(https://developer.android.com/reference/android/provider/Settings#ACTION_REQUEST_IGNORE_BATTERY_OPTIMIZATIONS)
    REQUEST_IGNORE_BATTERY_OPTIMIZATIONS,
    /// Allows an application to request installing packages
    REQUEST_INSTALL_PACKAGES,
    /// Allows an application to subscribe to notifications about the presence
    /// status change of their associated companion device
    REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE,
    /// Allows an application to request the screen lock complexity and prompt
    /// users to update the screen lock to a certain complexity level
    REQUEST_PASSWORD_COMPLEXITY,
    /// This constant was deprecated in API level 15
    ///
    /// The [`ActivityManager.restartPackage(String)`] API is no longer supported.
    ///
    /// [ActivityManager.restartPackage(String)]::(https://developer.android.com/reference/android/app/ActivityManager#restartPackage(java.lang.String))
    RESTART_PACKAGES,
    /// Allows applications to use exact alarm APIs
    SCHEDULE_EXACT_ALARM,
    /// Allows an application (Phone) to send a request to other applications to
    /// handle the respond-via-message action during incoming calls.
    SEND_RESPOND_VIA_MESSAGE,
    /// Allows an application to send SMS messages
    SEND_SMS,
    /// Allows an application to broadcast an Intent to set an alarm for the user
    SET_ALARM,
    /// Allows an application to control whether activities are immediately finished
    /// when put in the background
    SET_ALWAYS_FINISH,
    /// Modify the global animation scaling factor
    SET_ANIMATION_SCALE,
    /// Configure an application for debugging
    SET_DEBUG_APP,
    /// This constant was deprecated in API level 15. No longer useful, see
    /// [`PackageManager.addPackageToPreferred(String)`] for details.
    ///
    /// [PackageManager.addPackageToPreferred(String)]::(https://developer.android.com/reference/android/content/pm/PackageManager#addPackageToPreferred(java.lang.String))
    SET_PREFERRED_APPLICATIONS,
    /// Allows an application to set the maximum number of (not needed)
    /// application processes that can be running.
    SET_PROCESS_LIMIT,
    /// Allows applications to set the system time directly
    SET_TIME,
    /// Allows applications to set the system time zone directly
    SET_TIME_ZONE,
    /// Allows applications to set the wallpaper
    SET_WALLPAPER,
    /// Allows applications to set the wallpaper hints
    SET_WALLPAPER_HINTS,
    /// Allow an application to request that a signal be sent to all persistent processes
    SIGNAL_PERSISTENT_PROCESSES,
    /// This constant was deprecated in API level 31. The API that used this permission
    /// is no longer functional
    SMS_FINANCIAL_TRANSACTIONS,
    /// Allows an application to start foreground services from the background at any time
    START_FOREGROUND_SERVICES_FROM_BACKGROUND,
    /// Allows the holder to start the permission usage screen for an app
    START_VIEW_PERMISSION_USAGE,
    /// Allows an application to open, close, or disable the status bar and its icons
    STATUS_BAR,
    /// Allows an app to create windows using the type
    /// [`WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY`], shown on top of all
    /// other apps.
    ///
    /// [WindowManager.LayoutParams.TYPE_APPLICATION_OVERLAY]::(https://developer.android.com/reference/android/view/WindowManager.LayoutParams#TYPE_APPLICATION_OVERLAY)
    SYSTEM_ALERT_WINDOW,
    /// Allows using the device's IR transmitter, if available
    TRANSMIT_IR,
    /// Don't use this permission in your app
    UNINSTALL_SHORTCUT,
    /// Allows an application to update device statistics
    UPDATE_DEVICE_STATS,
    /// Allows an application to indicate via
    /// [`PackageInstaller.SessionParams.setRequireUserAction(int)`] that user action
    /// should not be required for an app update
    ///
    /// [PackageInstaller.SessionParams.setRequireUserAction(int)]::(https://developer.android.com/reference/android/content/pm/PackageInstaller.SessionParams#setRequireUserAction(int))
    UPDATE_PACKAGES_WITHOUT_USER_ACTION,
    /// Allows an app to use device supported biometric modalities
    USE_BIOMETRIC,
    /// This constant was deprecated in API level 28. Applications should request
    /// [`USE_BIOMETRIC`] instead
    ///
    /// [USE_BIOMETRIC]::(https://developer.android.com/reference/android/Manifest.permission#USE_BIOMETRIC)
    USE_FINGERPRINT,
    /// Required for apps targeting [`Build.VERSION_CODES.Q`] that want to use
    /// [`notification full screen intents`]
    ///
    /// [Build.VERSION_CODES.Q]::(https://developer.android.com/reference/android/os/Build.VERSION_CODES#Q)
    /// [notification full screen intents]::(https://developer.android.com/reference/android/app/Notification.Builder#setFullScreenIntent(android.app.PendingIntent,%20boolean))
    USE_FULL_SCREEN_INTENT,
    /// Allows to read device identifiers and use ICC based authentication like EAP-AKA
    USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER,
    /// Allows an application to use SIP service
    USE_SIP,
    /// Required to be able to range to devices using ultra-wideband
    UWB_RANGING,
    /// Allows access to the vibrator
    VIBRATE,
    /// Allows using PowerManager WakeLocks to keep processor from sleeping or screen
    /// from dimming
    WAKE_LOCK,
    /// Allows applications to write the apn settings and read sensitive fields of an
    /// existing apn settings like user and password
    WRITE_APN_SETTINGS,
    /// Allows an application to write the user's calendar data
    WRITE_CALENDAR,
    /// Allows an application to write (but not read) the user's call log data
    WRITE_CALL_LOG,
    /// Allows an application to write the user's contacts data
    WRITE_CONTACTS,
    /// Allows an application to write to external storage
    WRITE_EXTERNAL_STORAGE,
    /// Allows an application to modify the Google service map
    WRITE_GSERVICES,
    /// Allows an application to read or write the secure system settings
    WRITE_SECURE_SETTINGS,
    /// Allows an application to read or write the system settings
    WRITE_SETTINGS,
    /// Allows applications to write the sync settings
    WRITE_SYNC_SETTINGS,
    /// Allows an application to modify and remove existing voicemails in the system
    WRITE_VOICEMAIL,
}

impl AndroidPermission {
    pub fn get_full_permission(&self) -> String {
        "android.permission.".to_string() + self.to_string().as_str()
    }
}

impl std::fmt::Display for AndroidPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ACCEPT_HANDOVER => write!(f, "ACCEPT_HANDOVER"),
            Self::ACCESS_BACKGROUND_LOCATION => write!(f, "ACCESS_BACKGROUND_LOCATION"),
            Self::ACCESS_BLOBS_ACROSS_USERS => {
                write!(f, "ACCESS_BLOBS_ACROSS_USERS")
            }
            Self::ACCESS_CHECKIN_PROPERTIES => {
                write!(f, "ACCESS_CHECKIN_PROPERTIES")
            }
            Self::ACCESS_COARSE_LOCATION => write!(f, "ACCESS_COARSE_LOCATION"),
            Self::ACCESS_FINE_LOCATION => write!(f, "ACCESS_FINE_LOCATION"),
            Self::ACCESS_LOCATION_EXTRA_COMMANDS => {
                write!(f, "ACCESS_LOCATION_EXTRA_COMMANDS")
            }
            Self::ACCESS_MEDIA_LOCATION => write!(f, "ACCESS_MEDIA_LOCATION"),
            Self::ACCESS_NETWORK_STATE => write!(f, "ACCESS_NETWORK_STATE"),
            Self::ACCESS_NOTIFICATION_POLICY => {
                write!(f, "ACCESS_NOTIFICATION_POLICY")
            }
            Self::ACCESS_WIFI_STATE => write!(f, "ACCESS_WIFI_STATE"),
            Self::ACCOUNT_MANAGER => write!(f, "ACCOUNT_MANAGER"),
            Self::ACTIVITY_RECOGNITION => write!(f, "ACTIVITY_RECOGNITION"),
            Self::ADD_VOICEMAIL => write!(f, "ADD_VOICEMAIL"),
            Self::ANSWER_PHONE_CALLS => write!(f, "ANSWER_PHONE_CALLS"),
            Self::BATTERY_STATS => write!(f, "BATTERY_STATS"),
            Self::BIND_ACCESSIBILITY_SERVICE => {
                write!(f, "BIND_ACCESSIBILITY_SERVICE")
            }
            Self::BIND_APPWIDGET => write!(f, "BIND_APPWIDGET"),
            Self::BIND_AUTOFILL_SERVICE => write!(f, "BIND_AUTOFILL_SERVICE"),
            Self::BIND_CALL_REDIRECTION_SERVICE => {
                write!(f, "BIND_CALL_REDIRECTION_SERVICE")
            }
            Self::BIND_CARRIER_MESSAGING_CLIENT_SERVICE => {
                write!(f, "BIND_CARRIER_MESSAGING_CLIENT_SERVICE")
            }
            Self::BIND_CARRIER_MESSAGING_SERVICE => {
                write!(f, "BIND_CARRIER_MESSAGING_SERVICE")
            }
            Self::BIND_CARRIER_SERVICES => write!(f, "BIND_CARRIER_SERVICES"),
            Self::BIND_CHOOSER_TARGET_SERVICE => {
                write!(f, "BIND_CHOOSER_TARGET_SERVICE")
            }
            Self::BIND_COMPANION_DEVICE_SERVICE => {
                write!(f, "BIND_COMPANION_DEVICE_SERVICE")
            }
            Self::BIND_CONDITION_PROVIDER_SERVICE => {
                write!(f, "BIND_CONDITION_PROVIDER_SERVICE")
            }
            Self::BIND_CONTROLS => write!(f, "BIND_CONTROLS"),
            Self::BIND_DEVICE_ADMIN => write!(f, "BIND_DEVICE_ADMIN"),
            Self::BIND_DREAM_SERVICE => write!(f, "BIND_DREAM_SERVICE"),
            Self::BIND_INCALL_SERVICE => write!(f, "BIND_INCALL_SERVICE"),
            Self::BIND_INPUT_METHOD => write!(f, "BIND_INPUT_METHOD"),
            Self::BIND_MIDI_DEVICE_SERVICE => {
                write!(f, "BIND_MIDI_DEVICE_SERVICE")
            }
            Self::BIND_NFC_SERVICE => write!(f, "BIND_NFC_SERVICE"),
            Self::BIND_NOTIFICATION_LISTENER_SERVICE => {
                write!(f, "BIND_NOTIFICATION_LISTENER_SERVICE")
            }
            Self::BIND_PRINT_SERVICE => write!(f, "BIND_PRINT_SERVICE"),
            Self::BIND_QUICK_ACCESS_WALLET_SERVICE => {
                write!(f, "BIND_QUICK_ACCESS_WALLET_SERVICE")
            }
            Self::BIND_QUICK_SETTINGS_TILE => {
                write!(f, "BIND_QUICK_SETTINGS_TILE")
            }
            Self::BIND_REMOTEVIEWS => write!(f, "BIND_REMOTEVIEWS"),
            Self::BIND_SCREENING_SERVICE => write!(f, "BIND_SCREENING_SERVICE"),
            Self::BIND_TELECOM_CONNECTION_SERVICE => {
                write!(f, "BIND_TELECOM_CONNECTION_SERVICE")
            }
            Self::BIND_TEXT_SERVICE => write!(f, "BIND_TEXT_SERVICE"),
            Self::BIND_TV_INPUT => write!(f, "BIND_TV_INPUT"),
            Self::BIND_VISUAL_VOICEMAIL_SERVICE => {
                write!(f, "BIND_VISUAL_VOICEMAIL_SERVICE")
            }
            Self::BIND_VOICE_INTERACTION => write!(f, "BIND_VOICE_INTERACTION"),
            Self::BIND_VPN_SERVICE => write!(f, "BIND_VPN_SERVICE"),
            Self::BIND_VR_LISTENER_SERVICE => {
                write!(f, "BIND_VR_LISTENER_SERVICE")
            }
            Self::BIND_WALLPAPER => write!(f, "BIND_WALLPAPER"),
            Self::BLUETOOTH => write!(f, "BLUETOOTH"),
            Self::BLUETOOTH_ADMIN => write!(f, "BLUETOOTH_ADMIN"),
            Self::BLUETOOTH_ADVERTISE => write!(f, "BLUETOOTH_ADVERTISE"),
            Self::BLUETOOTH_CONNECT => write!(f, "BLUETOOTH_CONNECT"),
            Self::BLUETOOTH_PRIVILEGED => write!(f, "BLUETOOTH_PRIVILEGED"),
            Self::BLUETOOTH_SCAN => write!(f, "BLUETOOTH_SCAN"),
            Self::BODY_SENSORS => write!(f, "BODY_SENSORS"),
            Self::BROADCAST_PACKAGE_REMOVED => {
                write!(f, "BROADCAST_PACKAGE_REMOVED")
            }
            Self::BROADCAST_SMS => write!(f, "BROADCAST_SMS"),
            Self::BROADCAST_STICKY => write!(f, "BROADCAST_STICKY"),
            Self::BROADCAST_WAP_PUSH => write!(f, "BROADCAST_WAP_PUSH"),
            Self::CALL_COMPANION_APP => write!(f, "CALL_COMPANION_APP"),
            Self::CALL_PHONE => write!(f, "CALL_PHONE"),
            Self::CALL_PRIVILEGED => write!(f, "CALL_PRIVILEGED"),
            Self::CAMERA => write!(f, "CAMERA"),
            Self::CAPTURE_AUDIO_OUTPUT => write!(f, "CAPTURE_AUDIO_OUTPUT"),
            Self::CHANGE_COMPONENT_ENABLED_STATE => {
                write!(f, "CHANGE_COMPONENT_ENABLED_STATE")
            }
            Self::CHANGE_CONFIGURATION => write!(f, "CHANGE_CONFIGURATION"),
            Self::CHANGE_NETWORK_STATE => write!(f, "CHANGE_NETWORK_STATE"),
            Self::CHANGE_WIFI_MULTICAST_STATE => {
                write!(f, "CHANGE_WIFI_MULTICAST_STATE")
            }
            Self::CHANGE_WIFI_STATE => write!(f, "CHANGE_WIFI_STATE"),
            Self::CLEAR_APP_CACHE => write!(f, "CLEAR_APP_CACHE"),
            Self::CONTROL_LOCATION_UPDATES => {
                write!(f, "CONTROL_LOCATION_UPDATES")
            }
            Self::DELETE_CACHE_FILES => write!(f, "DELETE_CACHE_FILES"),
            Self::DELETE_PACKAGES => write!(f, "DELETE_PACKAGES"),
            Self::DIAGNOSTIC => write!(f, "DIAGNOSTIC"),
            Self::DISABLE_KEYGUARD => write!(f, "DISABLE_KEYGUARD"),
            Self::DUMP => write!(f, "DUMP"),
            Self::EXPAND_STATUS_BAR => write!(f, "EXPAND_STATUS_BAR"),
            Self::FACTORY_TEST => write!(f, "FACTORY_TEST"),
            Self::FOREGROUND_SERVICE => write!(f, "FOREGROUND_SERVICE"),
            Self::GET_ACCOUNTS => write!(f, "GET_ACCOUNTS"),
            Self::GET_ACCOUNTS_PRIVILEGED => {
                write!(f, "GET_ACCOUNTS_PRIVILEGED")
            }
            Self::GET_PACKAGE_SIZE => write!(f, "GET_PACKAGE_SIZE"),
            Self::GET_TASKS => write!(f, "GET_TASKS"),
            Self::GLOBAL_SEARCH => write!(f, "GLOBAL_SEARCH"),
            Self::HIDE_OVERLAY_WINDOWS => write!(f, "HIDE_OVERLAY_WINDOWS"),
            Self::HIGH_SAMPLING_RATE_SENSORS => {
                write!(f, "HIGH_SAMPLING_RATE_SENSORS")
            }
            Self::INSTALL_LOCATION_PROVIDER => {
                write!(f, "INSTALL_LOCATION_PROVIDER")
            }
            Self::INSTALL_PACKAGES => write!(f, "INSTALL_PACKAGES"),
            Self::INSTALL_SHORTCUT => write!(f, "INSTALL_SHORTCUT"),
            Self::INSTANT_APP_FOREGROUND_SERVICE => {
                write!(f, "INSTANT_APP_FOREGROUND_SERVICE")
            }
            Self::INTERACT_ACROSS_PROFILES => {
                write!(f, "INTERACT_ACROSS_PROFILES")
            }
            Self::INTERNET => write!(f, "INTERNET"),
            Self::KILL_BACKGROUND_PROCESSES => {
                write!(f, "KILL_BACKGROUND_PROCESSES")
            }
            Self::LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK => {
                write!(f, "LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK")
            }
            Self::LOADER_USAGE_STATS => write!(f, "LOADER_USAGE_STATS"),
            Self::LOCATION_HARDWARE => write!(f, "LOCATION_HARDWARE"),
            Self::MANAGE_DOCUMENTS => write!(f, "MANAGE_DOCUMENTS"),
            Self::MANAGE_EXTERNAL_STORAGE => {
                write!(f, "MANAGE_EXTERNAL_STORAGE")
            }
            Self::MANAGE_MEDIA => write!(f, "MANAGE_MEDIA"),
            Self::MANAGE_ONGOING_CALLS => write!(f, "MANAGE_ONGOING_CALLS"),
            Self::MANAGE_OWN_CALLS => write!(f, "MANAGE_OWN_CALLS"),
            Self::MASTER_CLEAR => write!(f, "MASTER_CLEAR"),
            Self::MEDIA_CONTENT_CONTROL => write!(f, "MEDIA_CONTENT_CONTROL"),
            Self::MODIFY_AUDIO_SETTINGS => write!(f, "MODIFY_AUDIO_SETTINGS"),
            Self::MODIFY_PHONE_STATE => write!(f, "MODIFY_PHONE_STATE"),
            Self::MOUNT_FORMAT_FILESYSTEMS => {
                write!(f, "MOUNT_FORMAT_FILESYSTEMS")
            }
            Self::MOUNT_UNMOUNT_FILESYSTEMS => {
                write!(f, "MOUNT_UNMOUNT_FILESYSTEMS")
            }
            Self::NFC => write!(f, "NFC"),
            Self::NFC_PREFERRED_PAYMENT_INFO => {
                write!(f, "NFC_PREFERRED_PAYMENT_INFO")
            }
            Self::NFC_TRANSACTION_EVENT => write!(f, "NFC_TRANSACTION_EVENT"),
            Self::PACKAGE_USAGE_STATS => write!(f, "PACKAGE_USAGE_STATS"),
            Self::PERSISTENT_ACTIVITY => write!(f, "PERSISTENT_ACTIVITY"),
            Self::PROCESS_OUTGOING_CALLS => write!(f, "PROCESS_OUTGOING_CALLS"),
            Self::QUERY_ALL_PACKAGES => write!(f, "QUERY_ALL_PACKAGES"),
            Self::READ_CALENDAR => write!(f, "READ_CALENDAR"),
            Self::READ_CALL_LOG => write!(f, "READ_CALL_LOG"),
            Self::READ_CONTACTS => write!(f, "READ_CONTACTS"),
            Self::READ_EXTERNAL_STORAG => write!(f, "READ_EXTERNAL_STORAG"),
            Self::READ_INPUT_STATE => write!(f, "READ_INPUT_STATE"),
            Self::READ_LOGS => write!(f, "READ_LOGS"),
            Self::READ_PHONE_NUMBERS => write!(f, "READ_PHONE_NUMBERS"),
            Self::READ_PHONE_STATE => write!(f, "READ_PHONE_STATE"),
            Self::READ_PRECISE_PHONE_STATE => {
                write!(f, "READ_PRECISE_PHONE_STATE")
            }
            Self::READ_SMS => write!(f, "READ_SMS"),
            Self::READ_SYNC_SETTINGS => write!(f, "READ_SYNC_SETTINGS"),
            Self::READ_SYNC_STATS => write!(f, "READ_SYNC_STATS"),
            Self::READ_VOICEMAIL => write!(f, "READ_VOICEMAIL"),
            Self::REBOOT => write!(f, "REBOOT"),
            Self::RECEIVE_BOOT_COMPLETED => write!(f, "RECEIVE_BOOT_COMPLETED"),
            Self::RECEIVE_MMS => write!(f, "RECEIVE_MMS"),
            Self::RECEIVE_SMS => write!(f, "RECEIVE_SMS"),
            Self::RECEIVE_WAP_PUSH => write!(f, "RECEIVE_WAP_PUSH"),
            Self::RECORD_AUDIO => write!(f, "RECORD_AUDIO"),
            Self::REORDER_TASKS => write!(f, "REORDER_TASKS"),
            Self::REQUEST_COMPANION_PROFILE_WATCH => write!(f, "REQUEST_COMPANION_PROFILE_WATCH"),
            Self::REQUEST_COMPANION_RUN_IN_BACKGROUND => {
                write!(f, "REQUEST_COMPANION_RUN_IN_BACKGROUND")
            }
            Self::REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND => write!(
                f,
                "REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND"
            ),
            Self::REQUEST_COMPANION_USE_DATA_IN_BACKGROUND => {
                write!(f, "REQUEST_COMPANION_USE_DATA_IN_BACKGROUND")
            }
            Self::REQUEST_DELETE_PACKAGES => {
                write!(f, "REQUEST_DELETE_PACKAGES")
            }
            Self::REQUEST_IGNORE_BATTERY_OPTIMIZATIONS => {
                write!(f, "REQUEST_IGNORE_BATTERY_OPTIMIZATIONS")
            }
            Self::REQUEST_INSTALL_PACKAGES => {
                write!(f, "REQUEST_INSTALL_PACKAGES")
            }
            Self::REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE => {
                write!(f, "REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE")
            }
            Self::REQUEST_PASSWORD_COMPLEXITY => {
                write!(f, "REQUEST_PASSWORD_COMPLEXITY")
            }
            Self::RESTART_PACKAGES => write!(f, "RESTART_PACKAGES"),
            Self::SCHEDULE_EXACT_ALARM => write!(f, "SCHEDULE_EXACT_ALARM"),
            Self::SEND_RESPOND_VIA_MESSAGE => {
                write!(f, "SEND_RESPOND_VIA_MESSAGE")
            }
            Self::SEND_SMS => write!(f, "SEND_SMS"),
            Self::SET_ALARM => write!(f, "SET_ALARM"),
            Self::SET_ALWAYS_FINISH => write!(f, "SET_ALWAYS_FINISH"),
            Self::SET_ANIMATION_SCALE => write!(f, "SET_ANIMATION_SCALE"),
            Self::SET_DEBUG_APP => write!(f, "SET_DEBUG_APP"),
            Self::SET_PREFERRED_APPLICATIONS => {
                write!(f, "SET_PREFERRED_APPLICATIONS")
            }
            Self::SET_PROCESS_LIMIT => write!(f, "SET_PROCESS_LIMIT"),
            Self::SET_TIME => write!(f, "SET_TIME"),
            Self::SET_TIME_ZONE => write!(f, "SET_TIME_ZONE"),
            Self::SET_WALLPAPER => write!(f, "SET_WALLPAPER"),
            Self::SET_WALLPAPER_HINTS => write!(f, "SET_WALLPAPER_HINTS"),
            Self::SIGNAL_PERSISTENT_PROCESSES => {
                write!(f, "SIGNAL_PERSISTENT_PROCESSES")
            }
            Self::SMS_FINANCIAL_TRANSACTIONS => {
                write!(f, "SMS_FINANCIAL_TRANSACTIONS")
            }
            Self::START_FOREGROUND_SERVICES_FROM_BACKGROUND => {
                write!(f, "START_FOREGROUND_SERVICES_FROM_BACKGROUND")
            }
            Self::START_VIEW_PERMISSION_USAGE => {
                write!(f, "START_VIEW_PERMISSION_USAGE")
            }
            Self::STATUS_BAR => write!(f, "STATUS_BAR"),
            Self::SYSTEM_ALERT_WINDOW => write!(f, "SYSTEM_ALERT_WINDOW"),
            Self::TRANSMIT_IR => write!(f, "TRANSMIT_IR"),
            Self::UNINSTALL_SHORTCUT => write!(f, "UNINSTALL_SHORTCUT"),
            Self::UPDATE_DEVICE_STATS => write!(f, "UPDATE_DEVICE_STATS"),
            Self::UPDATE_PACKAGES_WITHOUT_USER_ACTION => {
                write!(f, "UPDATE_PACKAGES_WITHOUT_USER_ACTION")
            }
            Self::USE_BIOMETRIC => write!(f, "USE_BIOMETRIC"),
            Self::USE_FINGERPRINT => write!(f, "USE_FINGERPRINT"),
            Self::USE_FULL_SCREEN_INTENT => write!(f, "USE_FULL_SCREEN_INTENT"),
            Self::USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER => {
                write!(f, "USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER")
            }
            Self::USE_SIP => write!(f, "USE_SIP"),
            Self::UWB_RANGING => write!(f, "UWB_RANGING"),
            Self::VIBRATE => write!(f, "VIBRATE"),
            Self::WAKE_LOCK => write!(f, "WAKE_LOCK"),
            Self::WRITE_APN_SETTINGS => write!(f, "WRITE_APN_SETTINGS"),
            Self::WRITE_CALENDAR => write!(f, "WRITE_CALENDAR"),
            Self::WRITE_CALL_LOG => write!(f, "WRITE_CALL_LOG"),
            Self::WRITE_CONTACTS => write!(f, "WRITE_CONTACTS"),
            Self::WRITE_EXTERNAL_STORAGE => write!(f, "WRITE_EXTERNAL_STORAGE"),
            Self::WRITE_GSERVICES => write!(f, "WRITE_GSERVICES"),
            Self::WRITE_SECURE_SETTINGS => write!(f, "WRITE_SECURE_SETTINGS"),
            Self::WRITE_SETTINGS => write!(f, "WRITE_SETTINGS"),
            Self::WRITE_SYNC_SETTINGS => write!(f, "WRITE_SYNC_SETTINGS"),
            Self::WRITE_VOICEMAIL => write!(f, "WRITE_VOICEMAIL"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_full_permission() {
        let permission = AndroidPermission::ACCESS_CHECKIN_PROPERTIES;
        assert_eq!(
            permission.get_full_permission(),
            "android.permission.ACCESS_CHECKIN_PROPERTIES"
        );
    }
}
