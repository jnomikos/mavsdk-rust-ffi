// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct EventOwned {
    pub compid: u32,
    pub message: String,
    pub description: String,
    pub log_level: crate::events::mavsdk::Events_LogLevel,
    pub event_namespace: String,
    pub event_name: String,
}

/*  */
pub struct Event<'a> {
    inner: &'a crate::events::mavsdk::Events_Event,
}

impl<'a> Event<'a> {
    pub fn new(inner: &'a crate::events::mavsdk::Events_Event) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> EventOwned {
        EventOwned {
            compid: self.compid(),
            message: self.message(),
            description: self.description(),
            log_level: self.log_level(),
            event_namespace: self.event_namespace(),
            event_name: self.event_name(),
        }
    }
    ///  The source component ID of the event

    pub fn compid(&self) -> u32 {
        self.inner.events_get_compid()
    }
    ///  Short, single-line message

    pub fn message(&self) -> String {
        self.inner
            .events_get_message()
            .to_string_lossy()
            .into_owned()
    }
    ///  Detailed description (optional, might be multiple lines)

    pub fn description(&self) -> String {
        self.inner
            .events_get_description()
            .to_string_lossy()
            .into_owned()
    }
    ///  Log level of message

    pub fn log_level(&self) -> crate::events::mavsdk::Events_LogLevel {
        self.inner.events_get_log_level().clone()
    }
    ///  Namespace, e.g. "px4"

    pub fn event_namespace(&self) -> String {
        self.inner
            .events_get_event_namespace()
            .to_string_lossy()
            .into_owned()
    }
    ///  Event name (unique within the namespace)

    pub fn event_name(&self) -> String {
        self.inner
            .events_get_event_name()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct HealthAndArmingCheckProblemOwned {
    pub message: String,
    pub description: String,
    pub log_level: crate::events::mavsdk::Events_LogLevel,
    pub health_component: String,
}

/*  */
pub struct HealthAndArmingCheckProblem<'a> {
    inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckProblem,
}

impl<'a> HealthAndArmingCheckProblem<'a> {
    pub fn new(inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckProblem) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HealthAndArmingCheckProblemOwned {
        HealthAndArmingCheckProblemOwned {
            message: self.message(),
            description: self.description(),
            log_level: self.log_level(),
            health_component: self.health_component(),
        }
    }
    ///  Short, single-line message

    pub fn message(&self) -> String {
        self.inner
            .events_get_message()
            .to_string_lossy()
            .into_owned()
    }
    ///  Detailed description (optional, might be multiple lines)

    pub fn description(&self) -> String {
        self.inner
            .events_get_description()
            .to_string_lossy()
            .into_owned()
    }
    ///  Log level of message

    pub fn log_level(&self) -> crate::events::mavsdk::Events_LogLevel {
        self.inner.events_get_log_level().clone()
    }
    ///  Associated health component, e.g. "gps"

    pub fn health_component(&self) -> String {
        self.inner
            .events_get_health_component()
            .to_string_lossy()
            .into_owned()
    }
}

pub struct HealthAndArmingCheckModeOwned {
    pub mode_name: String,
    pub can_arm_or_run: bool,
    pub problems: std::vec::Vec<HealthAndArmingCheckProblemOwned>,
}

/*  */
pub struct HealthAndArmingCheckMode<'a> {
    inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckMode,
}

impl<'a> HealthAndArmingCheckMode<'a> {
    pub fn new(inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckMode) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HealthAndArmingCheckModeOwned {
        HealthAndArmingCheckModeOwned {
            mode_name: self.mode_name(),
            can_arm_or_run: self.can_arm_or_run(),
            problems: self.problems(),
        }
    }
    ///  Mode name, e.g. "Position"

    pub fn mode_name(&self) -> String {
        self.inner
            .events_get_mode_name()
            .to_string_lossy()
            .into_owned()
    }
    ///  If disarmed: indicates if arming is possible. If armed: indicates if the mode can be selected

    pub fn can_arm_or_run(&self) -> bool {
        self.inner.events_get_can_arm_or_run()
    }
    ///  List of reported problems for the mode

    pub fn problems(&self) -> std::vec::Vec<HealthAndArmingCheckProblemOwned> {
        self.inner
            .events_get_problems()
            .iter()
            .map(|item| HealthAndArmingCheckProblem::new(item).into_owned())
            .collect()
    }
}

pub struct HealthComponentReportOwned {
    pub name: String,
    pub label: String,
    pub is_present: bool,
    pub has_error: bool,
    pub has_warning: bool,
}

/*  */
pub struct HealthComponentReport<'a> {
    inner: &'a crate::events::mavsdk::Events_HealthComponentReport,
}

impl<'a> HealthComponentReport<'a> {
    pub fn new(inner: &'a crate::events::mavsdk::Events_HealthComponentReport) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HealthComponentReportOwned {
        HealthComponentReportOwned {
            name: self.name(),
            label: self.label(),
            is_present: self.is_present(),
            has_error: self.has_error(),
            has_warning: self.has_warning(),
        }
    }
    ///  Unique component name, e.g. "gps"

    pub fn name(&self) -> String {
        self.inner.events_get_name().to_string_lossy().into_owned()
    }
    ///  Human readable label of the component, e.g. "GPS" or "Accelerometer"

    pub fn label(&self) -> String {
        self.inner.events_get_label().to_string_lossy().into_owned()
    }
    ///  If the component is present

    pub fn is_present(&self) -> bool {
        self.inner.events_get_is_present()
    }
    ///  If the component has errors

    pub fn has_error(&self) -> bool {
        self.inner.events_get_has_error()
    }
    ///  If the component has warnings

    pub fn has_warning(&self) -> bool {
        self.inner.events_get_has_warning()
    }
}

pub struct HealthAndArmingCheckReportOwned {
    pub current_mode_intention: HealthAndArmingCheckModeOwned,
    pub health_components: std::vec::Vec<HealthComponentReportOwned>,
    pub all_problems: std::vec::Vec<HealthAndArmingCheckProblemOwned>,
}

/*  */
pub struct HealthAndArmingCheckReport<'a> {
    inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckReport,
}

impl<'a> HealthAndArmingCheckReport<'a> {
    pub fn new(inner: &'a crate::events::mavsdk::Events_HealthAndArmingCheckReport) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> HealthAndArmingCheckReportOwned {
        HealthAndArmingCheckReportOwned {
            current_mode_intention: self.current_mode_intention(),
            health_components: self.health_components(),
            all_problems: self.all_problems(),
        }
    }
    ///  Report for currently intended mode

    pub fn current_mode_intention(&self) -> HealthAndArmingCheckModeOwned {
        HealthAndArmingCheckMode::new(self.inner.events_get_current_mode_intention()).into_owned()
    }
    ///  Health components list (e.g. for "gps")

    pub fn health_components(&self) -> std::vec::Vec<HealthComponentReportOwned> {
        self.inner
            .events_get_health_components()
            .iter()
            .map(|item| HealthComponentReport::new(item).into_owned())
            .collect()
    }
    ///  Complete list of problems

    pub fn all_problems(&self) -> std::vec::Vec<HealthAndArmingCheckProblemOwned> {
        self.inner
            .events_get_all_problems()
            .iter()
            .map(|item| HealthAndArmingCheckProblem::new(item).into_owned())
            .collect()
    }
}

struct EventsInner {
    plugin: cxx::UniquePtr<crate::events::mavsdk::Events>,

    /// Events State
    events_handle: Option<usize>,
    events_user_data: *mut c_void,
    /// HealthAndArmingChecks State
    health_and_arming_checks_handle: Option<usize>,
    health_and_arming_checks_user_data: *mut c_void,
}

pub struct EventsClient {
    inner: Mutex<EventsInner>,

    // Producer for the latest Events state; broadcast to all active subscribers.
    events_tx: watch::Sender<std::option::Option<EventOwned>>,
    // Producer for the latest HealthAndArmingChecks state; broadcast to all active subscribers.
    health_and_arming_checks_tx:
        watch::Sender<std::option::Option<HealthAndArmingCheckReportOwned>>,
}

impl EventsClient {
    pub fn new(plugin: cxx::UniquePtr<crate::events::mavsdk::Events>) -> Self {
        let (events_tx, _) = watch::channel(None);
        let (health_and_arming_checks_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(EventsInner {
                plugin,

                events_handle: None,
                events_user_data: std::ptr::null_mut(),
                health_and_arming_checks_handle: None,
                health_and_arming_checks_user_data: std::ptr::null_mut(),
            }),

            events_tx,
            health_and_arming_checks_tx,
        }
    }

    pub fn subscribe_events(&self) -> watch::Receiver<std::option::Option<EventOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.events_handle.is_none() {
            // Lazy initialization of C++ 'events' events stream.
            let user_data_ptr = Box::into_raw(Box::new(self.events_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_events(
                events_events_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.events_handle = Some(handle);
            inner.events_user_data = user_data_ptr;
        }

        self.events_tx.subscribe()
    }
    pub fn subscribe_health_and_arming_checks(
        &self,
    ) -> watch::Receiver<std::option::Option<HealthAndArmingCheckReportOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.health_and_arming_checks_handle.is_none() {
            // Lazy initialization of C++ 'health_and_arming_checks' events stream.
            let user_data_ptr =
                Box::into_raw(Box::new(self.health_and_arming_checks_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_health_and_arming_checks(
                events_health_and_arming_checks_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.health_and_arming_checks_handle = Some(handle);
            inner.health_and_arming_checks_user_data = user_data_ptr;
        }

        self.health_and_arming_checks_tx.subscribe()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn events_events_callback_ffi(
    user_data: *mut c_void,
    events: &crate::events::mavsdk::Events_Event,
) {
    if user_data.is_null() {
        return;
    }
    let send_events = Event::new(events).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<EventOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_events));
}
#[unsafe(no_mangle)]
pub extern "C" fn events_health_and_arming_checks_callback_ffi(
    user_data: *mut c_void,
    health_and_arming_checks: &crate::events::mavsdk::Events_HealthAndArmingCheckReport,
) {
    if user_data.is_null() {
        return;
    }
    let send_health_and_arming_checks =
        HealthAndArmingCheckReport::new(health_and_arming_checks).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe {
        &*(user_data as *const watch::Sender<std::option::Option<HealthAndArmingCheckReportOwned>>)
    };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_health_and_arming_checks));
}
