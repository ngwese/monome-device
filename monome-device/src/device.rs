//
// Copyright (c) 2024 Greg Wuller
//
// SPDX-License-Identifier: MIT
//

use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

use libmonome_sys as sys;
use url::Url;

use crate::event::Event;

#[derive(Debug)]
pub enum DeviceError {
    DeviceNameError(&'static str),
    OpenError(&'static str),
}

#[derive(Debug)]
pub enum DeviceRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl DeviceRotation {
    fn as_rotate_t(&self) -> sys::monome_rotate_t {
        match self {
            Self::Rotate0 => sys::monome_rotate_t_MONOME_ROTATE_0,
            Self::Rotate90 => sys::monome_rotate_t_MONOME_ROTATE_90,
            Self::Rotate180 => sys::monome_rotate_t_MONOME_ROTATE_180,
            Self::Rotate270 => sys::monome_rotate_t_MONOME_ROTATE_270,
        }
    }
}

impl From<sys::monome_rotate_t> for DeviceRotation {
    fn from(value: sys::monome_rotate_t) -> Self {
        match value {
            sys::monome_rotate_t_MONOME_ROTATE_0 => Self::Rotate0,
            sys::monome_rotate_t_MONOME_ROTATE_90 => Self::Rotate90,
            sys::monome_rotate_t_MONOME_ROTATE_180 => Self::Rotate180,
            sys::monome_rotate_t_MONOME_ROTATE_270 => Self::Rotate270,
            _ => Self::default(),
        }
    }
}

impl Default for DeviceRotation {
    fn default() -> Self {
        Self::Rotate0
    }
}

#[derive(Debug, Clone)]
struct DeviceHandle(*mut sys::monome);

unsafe impl Send for DeviceHandle {}
unsafe impl Sync for DeviceHandle {}

#[derive(Debug)]
pub struct Device {
    // identifier used to open the device
    device: String,

    serial: String,
    devpath: String,
    friendly_name: String,
    proto: String,
    rows: i32,
    cols: i32,

    handle: DeviceHandle,
}

impl Device {
    pub fn open_file(device_path: &str) -> Result<Self, DeviceError> {
        let raw_name = CString::new(device_path)
            .map_err(|_| DeviceError::DeviceNameError("unable to convert device string"))?;
        let handle = unsafe { sys::monome_open(raw_name.as_ptr()) };
        if handle.is_null() {
            Err(DeviceError::OpenError("open call failed"))
        } else {
            Self::new(device_path, DeviceHandle(handle))
        }
    }

    pub fn open_osc(_device: Url, _application_port: Option<u32>) -> Result<Self, DeviceError> {
        unimplemented!()
    }

    fn new(monome_device: &str, handle: DeviceHandle) -> Result<Self, DeviceError> {
        // gather all string based and/or contant properties, convert, and cache
        // to catch all potential conversion errors at contruction time.
        let raw_serial = unsafe { CStr::from_ptr(sys::monome_get_serial(handle.0)) };
        let serial = raw_serial
            .to_str()
            .map_err(|_| DeviceError::OpenError("unable to read serial"))?
            .to_string();

        let raw_devpath = unsafe { CStr::from_ptr(sys::monome_get_devpath(handle.0)) };
        let devpath = raw_devpath
            .to_str()
            .map_err(|_| DeviceError::OpenError("unable to read devpath"))?
            .to_string();

        let raw_friendly_name = unsafe { CStr::from_ptr(sys::monome_get_friendly_name(handle.0)) };
        let friendly_name = raw_friendly_name
            .to_str()
            .map_err(|_| DeviceError::OpenError("unable to read friendly_name"))?
            .to_string();

        let raw_proto = unsafe { CStr::from_ptr(sys::monome_get_proto(handle.0)) };
        let proto = raw_proto
            .to_str()
            .map_err(|_| DeviceError::OpenError("unable to read proto"))?
            .to_string();

        let rows = unsafe { sys::monome_get_rows(handle.0) };
        let cols = unsafe { sys::monome_get_cols(handle.0) };

        Ok(Self {
            device: monome_device.to_string(),

            serial,
            devpath,
            friendly_name,
            proto,
            rows,
            cols,

            handle,
        })
    }

    pub fn close(self) {
        unsafe { sys::monome_close(self.handle.0) };
    }

    pub fn event_loop(&self) -> EventLoop {
        EventLoop::start(self)
    }

    pub fn devname(&self) -> &str {
        &self.device
    }

    pub fn devpath(&self) -> &str {
        &self.devpath
    }

    pub fn serial(&self) -> &str {
        &self.serial
    }

    pub fn friendly_name(&self) -> &str {
        &self.friendly_name
    }

    pub fn proto(&self) -> &str {
        &self.proto
    }

    pub fn rows(&self) -> i32 {
        self.rows
    }

    pub fn cols(&self) -> i32 {
        self.cols
    }

    pub fn rotation_set(&self, r: DeviceRotation) {
        // NOTE: This is renamed for consistency with other _set suffix functions
        unsafe { sys::monome_set_rotation(self.handle.0, r.as_rotate_t()) }
    }

    pub fn rotation(&self) -> DeviceRotation {
        let r = unsafe { sys::monome_get_rotation(self.handle.0) };
        DeviceRotation::from(r)
    }

    pub fn tilt_enable(&self, sensor: u32) -> i32 {
        unsafe { sys::monome_tilt_enable(self.handle.0, sensor) }
    }

    pub fn tilt_disable(&self, sensor: u32) -> i32 {
        unsafe { sys::monome_tilt_enable(self.handle.0, sensor) }
    }

    pub fn led_set(&self, x: u32, y: u32, status: u32) -> i32 {
        unsafe { sys::monome_led_set(self.handle.0, x, y, status) }
    }

    pub fn led_on(&self, x: u32, y: u32) -> i32 {
        unsafe { sys::monome_led_on(self.handle.0, x, y) }
    }

    pub fn led_off(&self, x: u32, y: u32) -> i32 {
        unsafe { sys::monome_led_off(self.handle.0, x, y) }
    }

    pub fn led_all(&self, level: u32) -> i32 {
        unsafe { sys::monome_led_all(self.handle.0, level) }
    }

    pub fn led_map(&self, x_off: u32, y_off: u32, data: &[u8]) -> i32 {
        unsafe { sys::monome_led_map(self.handle.0, x_off, y_off, data.as_ptr()) }
    }

    pub fn led_col(&self, x: u32, y_off: u32, count: Option<usize>, data: &[u8]) -> i32 {
        let count = std::cmp::min(self.rows as usize, count.unwrap_or(data.len()));
        unsafe { sys::monome_led_col(self.handle.0, x, y_off, count, data.as_ptr()) }
    }

    pub fn led_row(&self, x_off: u32, y: u32, count: Option<usize>, data: &[u8]) -> i32 {
        let count = std::cmp::min(self.cols as usize, count.unwrap_or(data.len()));
        unsafe { sys::monome_led_row(self.handle.0, x_off, y, count, data.as_ptr()) }
    }

    pub fn led_intensity(&self, brightness: u32) -> i32 {
        unsafe { sys::monome_led_intensity(self.handle.0, brightness) }
    }

    pub fn led_level_set(&self, x: u32, y: u32, level: u32) -> i32 {
        unsafe { sys::monome_led_level_set(self.handle.0, x, y, level) }
    }

    pub fn led_level_all(&self, level: u32) -> i32 {
        unsafe { sys::monome_led_level_all(self.handle.0, level) }
    }

    pub fn led_level_map(&self, x_off: u32, y_off: u32, data: &[u8]) -> i32 {
        unsafe { sys::monome_led_level_map(self.handle.0, x_off, y_off, data.as_ptr()) }
    }

    pub fn led_level_col(&self, x: u32, y_off: u32, count: Option<usize>, data: &[u8]) -> i32 {
        let count = std::cmp::min(self.rows as usize, count.unwrap_or(data.len()));
        unsafe { sys::monome_led_level_col(self.handle.0, x, y_off, count, data.as_ptr()) }
    }

    pub fn led_level_row(&self, x_off: u32, y: u32, count: Option<usize>, data: &[u8]) -> i32 {
        let count = std::cmp::min(self.cols as usize, count.unwrap_or(data.len()));
        unsafe { sys::monome_led_level_row(self.handle.0, x_off, y, count, data.as_ptr()) }
    }

    pub fn led_ring_set(&self, ring: u32, led: u32, level: u32) -> i32 {
        unsafe { sys::monome_led_ring_set(self.handle.0, ring, led, level) }
    }

    pub fn led_ring_all(&self, ring: u32, level: u32) -> i32 {
        unsafe { sys::monome_led_ring_all(self.handle.0, ring, level) }
    }

    pub fn led_ring_map(&self, ring: u32, levels: &[u8]) -> i32 {
        unsafe { sys::monome_led_ring_map(self.handle.0, ring, levels.as_ptr()) }
    }

    pub fn led_ring_range(&self, ring: u32, start: u32, end: u32, level: u32) -> i32 {
        unsafe { sys::monome_led_ring_range(self.handle.0, ring, start, end, level) }
    }
}

#[no_mangle]
unsafe extern "C" fn monome_device_handle_button_press(
    event: *const sys::monome_event,
    data: *mut c_void,
) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let grid = (*event).__bindgen_anon_1.grid;
        let _ = tx.send(Event::ButtonPress {
            x: grid.x,
            y: grid.y,
        });
    }
}

#[no_mangle]
unsafe extern "C" fn monome_device_handle_button_lift(
    event: *const sys::monome_event,
    data: *mut c_void,
) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let grid = (*event).__bindgen_anon_1.grid;
        let _ = tx.send(Event::ButtonLift {
            x: grid.x,
            y: grid.y,
        });
    }
}

#[no_mangle]
extern "C" fn monome_device_handle_encoder_delta(
    event: *const sys::monome_event_t,
    data: *mut c_void,
) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let encoder = (*event).__bindgen_anon_1.encoder;
        let _ = tx.send(Event::EncoderDelta {
            number: encoder.number,
            delta: encoder.delta,
        });
    }
}

#[no_mangle]
extern "C" fn monome_device_handle_encoder_press(
    event: *const sys::monome_event_t,
    data: *mut c_void,
) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let encoder = (*event).__bindgen_anon_1.encoder;
        let _ = tx.send(Event::EncoderPress {
            number: encoder.number,
        });
    }
}

#[no_mangle]
extern "C" fn monome_device_handle_encoder_lift(
    event: *const sys::monome_event_t,
    data: *mut c_void,
) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let encoder = (*event).__bindgen_anon_1.encoder;
        let _ = tx.send(Event::EncoderLift {
            number: encoder.number,
        });
    }
}

#[no_mangle]
extern "C" fn monome_device_handle_tilt(event: *const sys::monome_event_t, data: *mut c_void) {
    unsafe {
        let tx = Box::leak(Box::from_raw(data as *mut mpsc::Sender<Event>));
        let tilt = (*event).__bindgen_anon_1.tilt;
        let _ = tx.send(Event::Tilt {
            sensor: tilt.sensor,
            x: tilt.x,
            y: tilt.y,
            z: tilt.z,
        });
    }
}

pub struct EventLoop<'a> {
    device: &'a Device,

    quit_tx: mpsc::Sender<()>,
    event_rx: mpsc::Receiver<Event>,

    join_handle: std::thread::JoinHandle<()>,
}

impl<'a> EventLoop<'a> {
    fn start(device: &'a Device) -> Self {
        // TODO: switch to bounded channels
        let (quit_tx, quit_rx) = mpsc::channel::<()>();
        let (event_tx, event_rx) = mpsc::channel::<Event>();

        // FIXME: This seems like super janky way to get a copy of the raw
        // monome device into another thread. Dark magic from:
        // https://users.rust-lang.org/t/how-to-share-a-raw-pointer-between-threads/77596
        let aliased_handle = DeviceHandle(device.handle.0);
        let join_handle = thread::spawn(move || {
            let _ = &aliased_handle;

            // TODO: Passing the channel endpoint into each callback each
            // callback is somewhat clunky, it would be nice to remove heap
            // allocation if possible.
            //
            // Within this closure the sender is moved onto the heap via
            // Box::new then consumed to produce a mutable pointer to the
            // contents. The mutable pointer is cast to a c_void pointer
            // compatible with the libmonome API. With each registered callback
            // unsafe operations are used to convert the raw pointer back into a
            // boxed value however a box owns it value therefore it is
            // explicitly leaked to prevent cleanup when the callback finishes.
            // Once the event loop finishes and the callbacks are unregistered
            // the raw pointer is converted once again into a boxed value with
            // is then correctly dropped.
            let event_tx = Box::into_raw(Box::new(event_tx));
            let user_data = event_tx as *mut c_void;

            // register handlers
            unsafe {
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_BUTTON_DOWN,
                    Some(monome_device_handle_button_press),
                    user_data,
                );
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_BUTTON_UP,
                    Some(monome_device_handle_button_lift),
                    user_data,
                );
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_DELTA,
                    Some(monome_device_handle_encoder_delta),
                    user_data,
                );
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_KEY_DOWN,
                    Some(monome_device_handle_encoder_press),
                    user_data,
                );
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_KEY_UP,
                    Some(monome_device_handle_encoder_lift),
                    user_data,
                );
                sys::monome_register_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_TILT,
                    Some(monome_device_handle_tilt),
                    user_data,
                );
            }

            // handle events until quit signal is received
            loop {
                let _status = unsafe { sys::monome_event_handle_next(aliased_handle.0) };
                match quit_rx.try_recv() {
                    Ok(_) => break,
                    Err(TryRecvError::Disconnected) => break,
                    _ => {}
                }
            }

            // cleanup handlers
            unsafe {
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_BUTTON_DOWN,
                );
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_BUTTON_UP,
                );
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_DELTA,
                );
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_KEY_DOWN,
                );
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_ENCODER_KEY_UP,
                );
                sys::monome_unregister_handler(
                    aliased_handle.0,
                    sys::monome_event_type_t_MONOME_TILT,
                );
            }

            // ensure event sender is dropped correctly
            let _event_tx = unsafe { Box::from_raw(event_tx) };
        });

        Self {
            device,
            event_rx,
            quit_tx,
            join_handle,
        }
    }

    pub fn stop(self) {
        let _ = self.quit_tx.send(());
        let _ = self.join_handle.join();
    }

    // pub fn next(&self) -> Option<Event> {
    //     self.event_rx.recv().ok()
    // }

    pub fn device(&self) -> &Device {
        self.device
    }
}

// impl<'a> Drop for EventLoop<'a> {
//     fn drop(&mut self) {
//         self.stop()
//     }
// }

impl<'a> Iterator for EventLoop<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        // skip over errors such as unhandled event types
        self.event_rx.recv().ok()
    }
}
