// Auto-generated wrapper. Do not edit.
use std::ffi::c_void;
use std::sync::Mutex;
use tokio::sync::watch;

pub struct StatusFlagsOwned {
    pub healthy: bool,
    pub fully_retracted: bool,
    pub moving: bool,
    pub clutch_engaged: bool,
    pub locked: bool,
    pub dropping: bool,
    pub arresting: bool,
    pub ground_sense: bool,
    pub retracting: bool,
    pub redeliver: bool,
    pub abandon_line: bool,
    pub locking: bool,
    pub load_line: bool,
    pub load_payload: bool,
}

/*  */
pub struct StatusFlags<'a> {
    inner: &'a crate::winch::mavsdk::Winch_StatusFlags,
}

impl<'a> StatusFlags<'a> {
    pub fn new(inner: &'a crate::winch::mavsdk::Winch_StatusFlags) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StatusFlagsOwned {
        StatusFlagsOwned {
            healthy: self.healthy(),
            fully_retracted: self.fully_retracted(),
            moving: self.moving(),
            clutch_engaged: self.clutch_engaged(),
            locked: self.locked(),
            dropping: self.dropping(),
            arresting: self.arresting(),
            ground_sense: self.ground_sense(),
            retracting: self.retracting(),
            redeliver: self.redeliver(),
            abandon_line: self.abandon_line(),
            locking: self.locking(),
            load_line: self.load_line(),
            load_payload: self.load_payload(),
        }
    }
    ///  Winch is healthy

    pub fn healthy(&self) -> bool {
        self.inner.winch_get_healthy()
    }
    ///  Winch line is fully retracted

    pub fn fully_retracted(&self) -> bool {
        self.inner.winch_get_fully_retracted()
    }
    ///  Winch motor is moving

    pub fn moving(&self) -> bool {
        self.inner.winch_get_moving()
    }
    ///  Winch clutch is engaged allowing motor to move freely

    pub fn clutch_engaged(&self) -> bool {
        self.inner.winch_get_clutch_engaged()
    }
    ///  Winch is locked by locking mechanism

    pub fn locked(&self) -> bool {
        self.inner.winch_get_locked()
    }
    ///  Winch is gravity dropping payload

    pub fn dropping(&self) -> bool {
        self.inner.winch_get_dropping()
    }
    ///  Winch is arresting payload descent

    pub fn arresting(&self) -> bool {
        self.inner.winch_get_arresting()
    }
    ///  Winch is using torque measurements to sense the ground

    pub fn ground_sense(&self) -> bool {
        self.inner.winch_get_ground_sense()
    }
    ///  Winch is returning to the fully retracted position

    pub fn retracting(&self) -> bool {
        self.inner.winch_get_retracting()
    }
    ///  Winch is redelivering the payload. This is a failover state if the line tension goes above a threshold during RETRACTING.

    pub fn redeliver(&self) -> bool {
        self.inner.winch_get_redeliver()
    }
    ///  Winch is abandoning the line and possibly payload. Winch unspools the entire calculated line length. This is a failover state from REDELIVER if the number of attempts exceeds a threshold.

    pub fn abandon_line(&self) -> bool {
        self.inner.winch_get_abandon_line()
    }
    ///  Winch is engaging the locking mechanism

    pub fn locking(&self) -> bool {
        self.inner.winch_get_locking()
    }
    ///  Winch is spooling on line

    pub fn load_line(&self) -> bool {
        self.inner.winch_get_load_line()
    }
    ///  Winch is loading a payload

    pub fn load_payload(&self) -> bool {
        self.inner.winch_get_load_payload()
    }
}

pub struct StatusOwned {
    pub time_usec: u64,
    pub line_length_m: f32,
    pub speed_m_s: f32,
    pub tension_kg: f32,
    pub voltage_v: f32,
    pub current_a: f32,
    pub temperature_c: i32,
    pub status_flags: StatusFlagsOwned,
}

/*  */
pub struct Status<'a> {
    inner: &'a crate::winch::mavsdk::Winch_Status,
}

impl<'a> Status<'a> {
    pub fn new(inner: &'a crate::winch::mavsdk::Winch_Status) -> Self {
        Self { inner }
    }

    pub fn into_owned(&self) -> StatusOwned {
        StatusOwned {
            time_usec: self.time_usec(),
            line_length_m: self.line_length_m(),
            speed_m_s: self.speed_m_s(),
            tension_kg: self.tension_kg(),
            voltage_v: self.voltage_v(),
            current_a: self.current_a(),
            temperature_c: self.temperature_c(),
            status_flags: self.status_flags(),
        }
    }
    ///  Time in usec

    pub fn time_usec(&self) -> u64 {
        self.inner.winch_get_time_usec()
    }
    ///  Length of the line in meters

    pub fn line_length_m(&self) -> f32 {
        self.inner.winch_get_line_length_m()
    }
    ///  Speed in meters per second

    pub fn speed_m_s(&self) -> f32 {
        self.inner.winch_get_speed_m_s()
    }
    ///  Tension in kilograms

    pub fn tension_kg(&self) -> f32 {
        self.inner.winch_get_tension_kg()
    }
    ///  Voltage in volts

    pub fn voltage_v(&self) -> f32 {
        self.inner.winch_get_voltage_v()
    }
    ///  Current in amperes

    pub fn current_a(&self) -> f32 {
        self.inner.winch_get_current_a()
    }
    ///  Temperature in Celsius

    pub fn temperature_c(&self) -> i32 {
        self.inner.winch_get_temperature_c()
    }
    ///  Status flags

    pub fn status_flags(&self) -> StatusFlagsOwned {
        StatusFlags::new(self.inner.winch_get_status_flags()).into_owned()
    }
}

struct WinchInner {
    plugin: cxx::UniquePtr<crate::winch::mavsdk::Winch>,

    /// Status State
    status_handle: Option<usize>,
    status_user_data: *mut c_void,
}

pub struct WinchClient {
    inner: Mutex<WinchInner>,

    // Producer for the latest Status state; broadcast to all active subscribers.
    status_tx: watch::Sender<std::option::Option<StatusOwned>>,
}

impl WinchClient {
    pub fn new(plugin: cxx::UniquePtr<crate::winch::mavsdk::Winch>) -> Self {
        let (status_tx, _) = watch::channel(None);

        Self {
            inner: Mutex::new(WinchInner {
                plugin,

                status_handle: None,
                status_user_data: std::ptr::null_mut(),
            }),

            status_tx,
        }
    }

    pub fn subscribe_status(&self) -> watch::Receiver<std::option::Option<StatusOwned>> {
        let mut inner = self.inner.lock().unwrap();

        if inner.status_handle.is_none() {
            // Lazy initialization of C++ 'status' winch stream.
            let user_data_ptr = Box::into_raw(Box::new(self.status_tx.clone())) as *mut c_void;

            let handle = inner.plugin.pin_mut().subscribe_status(
                winch_status_callback_ffi as usize as libc::uintptr_t,
                user_data_ptr as libc::uintptr_t,
            ) as usize;

            inner.status_handle = Some(handle);
            inner.status_user_data = user_data_ptr;
        }

        self.status_tx.subscribe()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn winch_status_callback_ffi(
    user_data: *mut c_void,
    status: &crate::winch::mavsdk::Winch_Status,
) {
    if user_data.is_null() {
        return;
    }
    let send_status = Status::new(status).into_owned();

    // Cast the pointer to watch::Sender
    let sender = unsafe { &*(user_data as *const watch::Sender<std::option::Option<StatusOwned>>) };

    // Overwrite the current value in the channel with the new update
    let _ = sender.send(Some(send_status));
}
