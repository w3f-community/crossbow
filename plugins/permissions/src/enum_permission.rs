#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permissions {
    /// Allows a calling app to continue a call which was started in another app
    ACCEPT_HANDOVER,
    /// Allows an app to access location in the background
    ACCESS_BACKGROUND_LOCATION,
    /// Allows an application to access data blobs across users
    ACCESS_BLOBS_ACROSS_USERS,
    /// Allows read/write access to the "properties" table in the checkin database,
    /// to change values that get uploaded
    ACCESS_CHECKIN_PROPERTIES,
    /// Allows an app to access approximate location
    ACCESS_COARSE_LOCATION,
    /// Allows an app to access precise location
    ACCESS_FINE_LOCATION,
    /// Allows an application to access extra location provider commands
    ACCESS_LOCATION_EXTRA_COMMANDS,
    /// Allows an application to access any geographic locations persisted in the
    /// user's shared collection.
    ACCESS_MEDIA_LOCATION,
    /// Allows applications to access information about networks
    ACCESS_NETWORK_STATE,
    /// Marker permission for applications that wish to access notification policy
    ACCESS_NOTIFICATION_POLICY,
    /// Allows applications to access information about Wi-Fi networks
    ACCESS_WIFI_STATE,
    /// Allows applications to call into AccountAuthenticators
    ACCOUNT_MANAGER,
    /// Allows an application to recognize physical activity
    ACTIVITY_RECOGNITION,
    /// Allows an application to add voicemails into the system
    ADD_VOICEMAIL,
    /// Allows the app to answer an incoming phone call
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
    WRITE_VOICEMAIL
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ACCEPT_HANDOVER => write!(f, "android.permission.ACCEPT_HANDOVER"),
            Self::ACCESS_BACKGROUND_LOCATION => write!(f, "android.permission.ACCESS_BACKGROUND_LOCATION"),
            Self::ACCESS_BLOBS_ACROSS_USERS => write!(f, "android.permission.ACCESS_BLOBS_ACROSS_USERS"),
            Self::ACCESS_CHECKIN_PROPERTIES => write!(f, "android.permission.ACCESS_CHECKIN_PROPERTIES"),
            Self::ACCESS_COARSE_LOCATION => write!(f, "android.permission.ACCESS_COARSE_LOCATION"),
            Self::ACCESS_FINE_LOCATION => write!(f, "android.permission.ACCESS_FINE_LOCATION"),
            Self::ACCESS_LOCATION_EXTRA_COMMANDS => write!(f, "android.permission.ACCESS_LOCATION_EXTRA_COMMANDS"),
            Self::ACCESS_MEDIA_LOCATION => write!(f, "android.permission.ACCESS_MEDIA_LOCATION"),
            Self::ACCESS_NETWORK_STATE => write!(f, "android.permission.ACCESS_NETWORK_STATE"),
            Self::ACCESS_NOTIFICATION_POLICY => write!(f, "android.permission.ACCESS_NOTIFICATION_POLICY"),
            Self::ACCESS_WIFI_STATE => write!(f, "android.permission.ACCESS_WIFI_STATE"),
            Self::ACCOUNT_MANAGER => write!(f, "android.permission.ACCOUNT_MANAGER"),
            Self::ACTIVITY_RECOGNITION => write!(f, "android.permission.ACTIVITY_RECOGNITION"),
            Self::ADD_VOICEMAIL => write!(f, "android.permission.ADD_VOICEMAIL"),
            Self::ANSWER_PHONE_CALLS => write!(f, "android.permission.ANSWER_PHONE_CALLS"),
            Self::BATTERY_STATS => write!(f, "android.permission.BATTERY_STATS"),
            Self::BIND_ACCESSIBILITY_SERVICE => write!(f, "android.permission.BIND_ACCESSIBILITY_SERVICE"),
            Self::BIND_APPWIDGET => write!(f, "android.permission.BIND_APPWIDGET"),
            Self::BIND_AUTOFILL_SERVICE => write!(f, "android.permission.BIND_AUTOFILL_SERVICE"),
            Self::BIND_CALL_REDIRECTION_SERVICE => write!(f, "android.permission.BIND_CALL_REDIRECTION_SERVICE"),
            Self::BIND_CARRIER_MESSAGING_CLIENT_SERVICE => write!(f, "android.permission.BIND_CARRIER_MESSAGING_CLIENT_SERVICE"),
            Self::BIND_CARRIER_MESSAGING_SERVICE => write!(f, "android.permission.BIND_CARRIER_MESSAGING_SERVICE"),
            Self::BIND_CARRIER_SERVICES => write!(f, "android.permission.BIND_CARRIER_SERVICES"),
            Self::BIND_CHOOSER_TARGET_SERVICE => write!(f, "android.permission.BIND_CHOOSER_TARGET_SERVICE"),
            Self::BIND_COMPANION_DEVICE_SERVICE => write!(f, "android.permission.BIND_COMPANION_DEVICE_SERVICE"),
            Self::BIND_CONDITION_PROVIDER_SERVICE => write!(f, "android.permission.BIND_CONDITION_PROVIDER_SERVICE"),
            Self::BIND_CONTROLS => write!(f, "android.permission.BIND_CONTROLS"),
            Self::BIND_DEVICE_ADMIN => write!(f, "android.permission.BIND_DEVICE_ADMIN"),
            Self::BIND_DREAM_SERVICE => write!(f, "android.permission.BIND_DREAM_SERVICE"),
            Self::BIND_INCALL_SERVICE => write!(f, "android.permission.BIND_INCALL_SERVICE"),
            Self::BIND_INPUT_METHOD => write!(f, "android.permission.BIND_INPUT_METHOD"),
            Self::BIND_MIDI_DEVICE_SERVICE => write!(f, "android.permission.BIND_MIDI_DEVICE_SERVICE"),
            Self::BIND_NFC_SERVICE => write!(f, "android.permission.BIND_NFC_SERVICE"),
            Self::BIND_NOTIFICATION_LISTENER_SERVICE => write!(f, "android.permission.BIND_NOTIFICATION_LISTENER_SERVICE"),
            Self::BIND_PRINT_SERVICE => write!(f, "android.permission.BIND_PRINT_SERVICE"),
            Self::BIND_QUICK_ACCESS_WALLET_SERVICE => write!(f, "android.permission.BIND_QUICK_ACCESS_WALLET_SERVICE"),
            Self::BIND_QUICK_SETTINGS_TILE => write!(f, "android.permission.BIND_QUICK_SETTINGS_TILE"),
            Self::BIND_REMOTEVIEWS => write!(f, "android.permission.BIND_REMOTEVIEWS"),
            Self::BIND_SCREENING_SERVICE => write!(f, "android.permission.BIND_SCREENING_SERVICE"),
            Self::BIND_TELECOM_CONNECTION_SERVICE => write!(f, "android.permission.BIND_TELECOM_CONNECTION_SERVICE"),
            Self::BIND_TEXT_SERVICE => write!(f, "android.permission.BIND_TEXT_SERVICE"),
            Self::BIND_TV_INPUT => write!(f, "android.permission.BIND_TV_INPUT"),
            Self::BIND_VISUAL_VOICEMAIL_SERVICE => write!(f, "android.permission.BIND_VISUAL_VOICEMAIL_SERVICE"),
            Self::BIND_VOICE_INTERACTION => write!(f, "android.permission.BIND_VOICE_INTERACTION"),
            Self::BIND_VPN_SERVICE => write!(f, "android.permission.BIND_VPN_SERVICE"),
            Self::BIND_VR_LISTENER_SERVICE => write!(f, "android.permission.BIND_VR_LISTENER_SERVICE"),
            Self::BIND_WALLPAPER => write!(f, "android.permission.BIND_WALLPAPER"),
            Self::BLUETOOTH => write!(f, "android.permission.BLUETOOTH"),
            Self::BLUETOOTH_ADMIN => write!(f, "android.permission.BLUETOOTH_ADMIN"),
            Self::BLUETOOTH_ADVERTISE => write!(f, "android.permission.BLUETOOTH_ADVERTISE"),
            Self::BLUETOOTH_CONNECT => write!(f, "android.permission.BLUETOOTH_CONNECT"),
            Self::BLUETOOTH_PRIVILEGED => write!(f, "android.permission.BLUETOOTH_PRIVILEGED"),
            Self::BLUETOOTH_SCAN => write!(f, "android.permission.BLUETOOTH_SCAN"),
            Self::BODY_SENSORS => write!(f, "android.permission.BODY_SENSORS"),
            Self::BROADCAST_PACKAGE_REMOVED => write!(f, "android.permission.BROADCAST_PACKAGE_REMOVED"),
            Self::BROADCAST_SMS => write!(f, "android.permission.BROADCAST_SMS"),
            Self::BROADCAST_STICKY => write!(f, "android.permission.BROADCAST_STICKY"),
            Self::BROADCAST_WAP_PUSH => write!(f, "android.permission.BROADCAST_WAP_PUSH"),
            Self::CALL_COMPANION_APP => write!(f, "android.permission.CALL_COMPANION_APP"),
            Self::CALL_PHONE => write!(f, "android.permission.CALL_PHONE"),
            Self::CALL_PRIVILEGED => write!(f, "android.permission.CALL_PRIVILEGED"),
            Self::CAMERA => write!(f, "android.permission.CAMERA"),
            Self::CAPTURE_AUDIO_OUTPUT => write!(f, "android.permission.CAPTURE_AUDIO_OUTPUT"),
            Self::CHANGE_COMPONENT_ENABLED_STATE => write!(f, "android.permission.CHANGE_COMPONENT_ENABLED_STATE"),
            Self::CHANGE_CONFIGURATION => write!(f, "android.permission.CHANGE_CONFIGURATION"),
            Self::CHANGE_NETWORK_STATE => write!(f, "android.permission.CHANGE_NETWORK_STATE"),
            Self::CHANGE_WIFI_MULTICAST_STATE => write!(f, "android.permission.CHANGE_WIFI_MULTICAST_STATE"),
            Self::CHANGE_WIFI_STATE => write!(f, "android.permission.CHANGE_WIFI_STATE"),
            Self::CLEAR_APP_CACHE => write!(f, "android.permission.CLEAR_APP_CACHE"),
            Self::CONTROL_LOCATION_UPDATES => write!(f, "android.permission.CONTROL_LOCATION_UPDATES"),
            Self::DELETE_CACHE_FILES => write!(f, "android.permission.DELETE_CACHE_FILES"),
            Self::DELETE_PACKAGES => write!(f, "android.permission.DELETE_PACKAGES"),
            Self::DIAGNOSTIC => write!(f, "android.permission.DIAGNOSTIC"),
            Self::DISABLE_KEYGUARD => write!(f, "android.permission.DISABLE_KEYGUARD"),
            Self::DUMP => write!(f, "android.permission.DUMP"),
            Self::EXPAND_STATUS_BAR => write!(f, "android.permission.EXPAND_STATUS_BAR"),
            Self::FACTORY_TEST => write!(f, "android.permission.FACTORY_TEST"),
            Self::FOREGROUND_SERVICE => write!(f, "android.permission.FOREGROUND_SERVICE"),
            Self::GET_ACCOUNTS => write!(f, "android.permission.GET_ACCOUNTS"),
            Self::GET_ACCOUNTS_PRIVILEGED => write!(f, "android.permission.GET_ACCOUNTS_PRIVILEGED"),
            Self::GET_PACKAGE_SIZE => write!(f, "android.permission.GET_PACKAGE_SIZE"),
            Self::GET_TASKS => write!(f, "android.permission.GET_TASKS"),
            Self::GLOBAL_SEARCH => write!(f, "android.permission.GLOBAL_SEARCH"),
            Self::HIDE_OVERLAY_WINDOWS => write!(f, "android.permission.HIDE_OVERLAY_WINDOWS"),
            Self::HIGH_SAMPLING_RATE_SENSORS => write!(f, "android.permission.HIGH_SAMPLING_RATE_SENSORS"),
            Self::INSTALL_LOCATION_PROVIDER => write!(f, "android.permission.INSTALL_LOCATION_PROVIDER"),
            Self::INSTALL_PACKAGES => write!(f, "android.permission.INSTALL_PACKAGES"),
            Self::INSTALL_SHORTCUT => write!(f, "android.permission.INSTALL_SHORTCUT"),
            Self::INSTANT_APP_FOREGROUND_SERVICE => write!(f, "android.permission.INSTANT_APP_FOREGROUND_SERVICE"),
            Self::INTERACT_ACROSS_PROFILES => write!(f, "android.permission.INTERACT_ACROSS_PROFILES"),
            Self::INTERNET => write!(f, "android.permission.INTERNET"),
            Self::KILL_BACKGROUND_PROCESSES => write!(f, "android.permission.KILL_BACKGROUND_PROCESSES"),
            Self::LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK => write!(f, "android.permission.LAUNCH_MULTI_PANE_SETTINGS_DEEP_LINK"),
            Self::LOADER_USAGE_STATS => write!(f, "android.permission.LOADER_USAGE_STATS"),
            Self::LOCATION_HARDWARE => write!(f, "android.permission.LOCATION_HARDWARE"),
            Self::MANAGE_DOCUMENTS => write!(f, "android.permission.MANAGE_DOCUMENTS"),
            Self::MANAGE_EXTERNAL_STORAGE => write!(f, "android.permission.MANAGE_EXTERNAL_STORAGE"),
            Self::MANAGE_MEDIA => write!(f, "android.permission.MANAGE_MEDIA"),
            Self::MANAGE_ONGOING_CALLS => write!(f, "android.permission.MANAGE_ONGOING_CALLS"),
            Self::MANAGE_OWN_CALLS => write!(f, "android.permission.MANAGE_OWN_CALLS"),
            Self::MASTER_CLEAR => write!(f, "android.permission.MASTER_CLEAR"),
            Self::MEDIA_CONTENT_CONTROL => write!(f, "android.permission.MEDIA_CONTENT_CONTROL"),
            Self::MODIFY_AUDIO_SETTINGS => write!(f, "android.permission.MODIFY_AUDIO_SETTINGS"),
            Self::MODIFY_PHONE_STATE => write!(f, "android.permission.MODIFY_PHONE_STATE"),
            Self::MOUNT_FORMAT_FILESYSTEMS => write!(f, "android.permission.MOUNT_FORMAT_FILESYSTEMS"),
            Self::MOUNT_UNMOUNT_FILESYSTEMS => write!(f, "android.permission.MOUNT_UNMOUNT_FILESYSTEMS"),
            Self::NFC => write!(f, "android.permission.NFC"),
            Self::NFC_PREFERRED_PAYMENT_INFO => write!(f, "android.permission.NFC_PREFERRED_PAYMENT_INFO"),
            Self::NFC_TRANSACTION_EVENT => write!(f, "android.permission.NFC_TRANSACTION_EVENT"),
            Self::PACKAGE_USAGE_STATS => write!(f, "android.permission.PACKAGE_USAGE_STATS"),
            Self::PERSISTENT_ACTIVITY => write!(f, "android.permission.PERSISTENT_ACTIVITY"),
            Self::PROCESS_OUTGOING_CALLS => write!(f, "android.permission.PROCESS_OUTGOING_CALLS"),
            Self::QUERY_ALL_PACKAGES => write!(f, "android.permission.QUERY_ALL_PACKAGES"),
            Self::READ_CALENDAR => write!(f, "android.permission.READ_CALENDAR"),
            Self::READ_CALL_LOG => write!(f, "android.permission.READ_CALL_LOG"),
            Self::READ_CONTACTS => write!(f, "android.permission.READ_CONTACTS"),
            Self::READ_EXTERNAL_STORAG => write!(f, "android.permission.READ_EXTERNAL_STORAG"),
            Self::READ_INPUT_STATE => write!(f, "android.permission.READ_INPUT_STATE"),
            Self::READ_LOGS => write!(f, "android.permission.READ_LOGS"),
            Self::READ_PHONE_NUMBERS => write!(f, "android.permission.READ_PHONE_NUMBERS"),
            Self::READ_PHONE_STATE => write!(f, "android.permission.READ_PHONE_STATE"),
            Self::READ_PRECISE_PHONE_STATE => write!(f, "android.permission.READ_PRECISE_PHONE_STATE"),
            Self::READ_SMS => write!(f, "android.permission.READ_SMS"),
            Self::READ_SYNC_SETTINGS => write!(f, "android.permission.READ_SYNC_SETTINGS"),
            Self::READ_SYNC_STATS => write!(f, "android.permission.READ_SYNC_STATS"),
            Self::READ_VOICEMAIL => write!(f, "android.permission.READ_VOICEMAIL"),
            Self::REBOOT => write!(f, "android.permission.REBOOT"),
            Self::RECEIVE_BOOT_COMPLETED => write!(f, "android.permission.RECEIVE_BOOT_COMPLETED"),
            Self::RECEIVE_MMS => write!(f, "android.permission.RECEIVE_MMS"),
            Self::RECEIVE_SMS => write!(f, "android.permission.RECEIVE_SMS"),
            Self::RECEIVE_WAP_PUSH => write!(f, "android.permission.RECEIVE_WAP_PUSH"),
            Self::RECORD_AUDIO => write!(f, "android.permission.RECORD_AUDIO"),
            Self::REORDER_TASKS => write!(f, "android.permission.REORDER_TASKS"),
            Self::REQUEST_COMPANION_PROFILE_WATCH => write!(f, "android.permission.REQUEST_COMPANION_PROFILE_WATCH"),
            Self::REQUEST_COMPANION_RUN_IN_BACKGROUND => write!(f, "android.permission.REQUEST_COMPANION_RUN_IN_BACKGROUND"),
            Self::REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND => write!(f, "android.permission.REQUEST_COMPANION_START_FOREGROUND_SERVICES_FROM_BACKGROUND"),
            Self::REQUEST_COMPANION_USE_DATA_IN_BACKGROUND => write!(f, "android.permission.REQUEST_COMPANION_USE_DATA_IN_BACKGROUND"),
            Self::REQUEST_DELETE_PACKAGES => write!(f, "android.permission.REQUEST_DELETE_PACKAGES"),
            Self::REQUEST_IGNORE_BATTERY_OPTIMIZATIONS => write!(f, "android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS"),
            Self::REQUEST_INSTALL_PACKAGES => write!(f, "android.permission.REQUEST_INSTALL_PACKAGES"),
            Self::REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE => write!(f, "android.permission.REQUEST_OBSERVE_COMPANION_DEVICE_PRESENCE"),
            Self::REQUEST_PASSWORD_COMPLEXITY => write!(f, "android.permission.REQUEST_PASSWORD_COMPLEXITY"),
            Self::RESTART_PACKAGES => write!(f, "android.permission.RESTART_PACKAGES"),
            Self::SCHEDULE_EXACT_ALARM => write!(f, "android.permission.SCHEDULE_EXACT_ALARM"),
            Self::SEND_RESPOND_VIA_MESSAGE => write!(f, "android.permission.SEND_RESPOND_VIA_MESSAGE"),
            Self::SEND_SMS => write!(f, "android.permission.SEND_SMS"),
            Self::SET_ALARM => write!(f, "android.permission.SET_ALARM"),
            Self::SET_ALWAYS_FINISH => write!(f, "android.permission.SET_ALWAYS_FINISH"),
            Self::SET_ANIMATION_SCALE => write!(f, "android.permission.SET_ANIMATION_SCALE"),
            Self::SET_DEBUG_APP => write!(f, "android.permission.SET_DEBUG_APP"),
            Self::SET_PREFERRED_APPLICATIONS => write!(f, "android.permission.SET_PREFERRED_APPLICATIONS"),
            Self::SET_PROCESS_LIMIT => write!(f, "android.permission.SET_PROCESS_LIMIT"),
            Self::SET_TIME => write!(f, "android.permission.SET_TIME"),
            Self::SET_TIME_ZONE => write!(f, "android.permission.SET_TIME_ZONE"),
            Self::SET_WALLPAPER => write!(f, "android.permission.SET_WALLPAPER"),
            Self::SET_WALLPAPER_HINTS => write!(f, "android.permission.SET_WALLPAPER_HINTS"),
            Self::SIGNAL_PERSISTENT_PROCESSES => write!(f, "android.permission.SIGNAL_PERSISTENT_PROCESSES"),
            Self::SMS_FINANCIAL_TRANSACTIONS => write!(f, "android.permission.SMS_FINANCIAL_TRANSACTIONS"),
            Self::START_FOREGROUND_SERVICES_FROM_BACKGROUND => write!(f, "android.permission.START_FOREGROUND_SERVICES_FROM_BACKGROUND"),
            Self::START_VIEW_PERMISSION_USAGE => write!(f, "android.permission.START_VIEW_PERMISSION_USAGE"),
            Self::STATUS_BAR => write!(f, "android.permission.STATUS_BAR"),
            Self::SYSTEM_ALERT_WINDOW => write!(f, "android.permission.SYSTEM_ALERT_WINDOW"),
            Self::TRANSMIT_IR => write!(f, "android.permission.TRANSMIT_IR"),
            Self::UNINSTALL_SHORTCUT => write!(f, "android.permission.UNINSTALL_SHORTCUT"),
            Self::UPDATE_DEVICE_STATS => write!(f, "android.permission.UPDATE_DEVICE_STATS"),
            Self::UPDATE_PACKAGES_WITHOUT_USER_ACTION => write!(f, "android.permission.UPDATE_PACKAGES_WITHOUT_USER_ACTION"),
            Self::USE_BIOMETRIC => write!(f, "android.permission.USE_BIOMETRIC"),
            Self::USE_FINGERPRINT => write!(f, "android.permission.USE_FINGERPRINT"),
            Self::USE_FULL_SCREEN_INTENT => write!(f, "android.permission.USE_FULL_SCREEN_INTENT"),
            Self::USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER => write!(f, "android.permission.USE_ICC_AUTH_WITH_DEVICE_IDENTIFIER"),
            Self::USE_SIP => write!(f, "android.permission.USE_SIP"),
            Self::UWB_RANGING => write!(f, "android.permission.UWB_RANGING"),
            Self::VIBRATE => write!(f, "android.permission.VIBRATE"),
            Self::WAKE_LOCK => write!(f, "android.permission.WAKE_LOCK"),
            Self::WRITE_APN_SETTINGS => write!(f, "android.permission.WRITE_APN_SETTINGS"),
            Self::WRITE_CALENDAR => write!(f, "android.permission.WRITE_CALENDAR"),
            Self::WRITE_CALL_LOG => write!(f, "android.permission.WRITE_CALL_LOG"),
            Self::WRITE_CONTACTS => write!(f, "android.permission.WRITE_CONTACTS"),
            Self::WRITE_EXTERNAL_STORAGE => write!(f, "android.permission.WRITE_EXTERNAL_STORAGE"),
            Self::WRITE_GSERVICES => write!(f, "android.permission.WRITE_GSERVICES"),
            Self::WRITE_SECURE_SETTINGS => write!(f, "android.permission.WRITE_SECURE_SETTINGS"),
            Self::WRITE_SETTINGS => write!(f, "android.permission.WRITE_SETTINGS"),
            Self::WRITE_SYNC_SETTINGS => write!(f, "android.permission.WRITE_SYNC_SETTINGS"),
            Self::WRITE_VOICEMAIL => write!(f, "android.permission.WRITE_VOICEMAIL"),
        }
    }
}