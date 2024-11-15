/* automatically generated by rust-bindgen 0.70.1 */

pub const monome_event_type_t_MONOME_BUTTON_UP: monome_event_type_t = 0;
pub const monome_event_type_t_MONOME_BUTTON_DOWN: monome_event_type_t = 1;
pub const monome_event_type_t_MONOME_ENCODER_DELTA: monome_event_type_t = 2;
pub const monome_event_type_t_MONOME_ENCODER_KEY_UP: monome_event_type_t = 3;
pub const monome_event_type_t_MONOME_ENCODER_KEY_DOWN: monome_event_type_t = 4;
pub const monome_event_type_t_MONOME_TILT: monome_event_type_t = 5;
pub const monome_event_type_t_MONOME_EVENT_MAX: monome_event_type_t = 6;
pub type monome_event_type_t = ::std::os::raw::c_uint;
pub const monome_rotate_t_MONOME_ROTATE_0: monome_rotate_t = 0;
pub const monome_rotate_t_MONOME_ROTATE_90: monome_rotate_t = 1;
pub const monome_rotate_t_MONOME_ROTATE_180: monome_rotate_t = 2;
pub const monome_rotate_t_MONOME_ROTATE_270: monome_rotate_t = 3;
pub type monome_rotate_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct monome {
    _unused: [u8; 0],
}
pub type monome_t = monome;
pub type monome_event_t = monome_event;
pub type monome_event_callback_t = ::std::option::Option<
    unsafe extern "C" fn(event: *const monome_event_t, data: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct monome_event {
    pub monome: *mut monome_t,
    pub event_type: monome_event_type_t,
    pub __bindgen_anon_1: monome_event__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union monome_event__bindgen_ty_1 {
    pub grid: monome_event__bindgen_ty_1__bindgen_ty_1,
    pub encoder: monome_event__bindgen_ty_1__bindgen_ty_2,
    pub tilt: monome_event__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct monome_event__bindgen_ty_1__bindgen_ty_1 {
    pub x: ::std::os::raw::c_uint,
    pub y: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of monome_event__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<monome_event__bindgen_ty_1__bindgen_ty_1>() - 8usize];
    ["Alignment of monome_event__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<monome_event__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_1::x"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_1, x) - 0usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_1::y"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_1, y) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct monome_event__bindgen_ty_1__bindgen_ty_2 {
    pub number: ::std::os::raw::c_uint,
    pub delta: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of monome_event__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::size_of::<monome_event__bindgen_ty_1__bindgen_ty_2>() - 8usize];
    ["Alignment of monome_event__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::align_of::<monome_event__bindgen_ty_1__bindgen_ty_2>() - 4usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_2::number"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_2, number) - 0usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_2::delta"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_2, delta) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct monome_event__bindgen_ty_1__bindgen_ty_3 {
    pub sensor: ::std::os::raw::c_uint,
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
    pub z: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of monome_event__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::size_of::<monome_event__bindgen_ty_1__bindgen_ty_3>() - 16usize];
    ["Alignment of monome_event__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::align_of::<monome_event__bindgen_ty_1__bindgen_ty_3>() - 4usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_3::sensor"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_3, sensor) - 0usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_3::x"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_3, x) - 4usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_3::y"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_3, y) - 8usize];
    ["Offset of field: monome_event__bindgen_ty_1__bindgen_ty_3::z"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1__bindgen_ty_3, z) - 12usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of monome_event__bindgen_ty_1"]
        [::std::mem::size_of::<monome_event__bindgen_ty_1>() - 16usize];
    ["Alignment of monome_event__bindgen_ty_1"]
        [::std::mem::align_of::<monome_event__bindgen_ty_1>() - 4usize];
    ["Offset of field: monome_event__bindgen_ty_1::grid"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1, grid) - 0usize];
    ["Offset of field: monome_event__bindgen_ty_1::encoder"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1, encoder) - 0usize];
    ["Offset of field: monome_event__bindgen_ty_1::tilt"]
        [::std::mem::offset_of!(monome_event__bindgen_ty_1, tilt) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of monome_event"][::std::mem::size_of::<monome_event>() - 32usize];
    ["Alignment of monome_event"][::std::mem::align_of::<monome_event>() - 8usize];
    ["Offset of field: monome_event::monome"]
        [::std::mem::offset_of!(monome_event, monome) - 0usize];
    ["Offset of field: monome_event::event_type"]
        [::std::mem::offset_of!(monome_event, event_type) - 8usize];
};
extern "C" {
    pub fn monome_open(monome_device: *const ::std::os::raw::c_char, ...) -> *mut monome_t;
}
extern "C" {
    pub fn monome_close(monome: *mut monome_t);
}
extern "C" {
    pub fn monome_set_rotation(monome: *mut monome_t, cable: monome_rotate_t);
}
extern "C" {
    pub fn monome_get_rotation(monome: *mut monome_t) -> monome_rotate_t;
}
extern "C" {
    pub fn monome_get_serial(monome: *mut monome_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn monome_get_devpath(monome: *mut monome_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn monome_get_friendly_name(monome: *mut monome_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn monome_get_proto(monome: *mut monome_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn monome_get_rows(monome: *mut monome_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_get_cols(monome: *mut monome_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_register_handler(
        monome: *mut monome_t,
        event_type: monome_event_type_t,
        arg1: monome_event_callback_t,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_unregister_handler(
        monome: *mut monome_t,
        event_type: monome_event_type_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_event_next(
        monome: *mut monome_t,
        event_buf: *mut monome_event_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_event_handle_next(monome: *mut monome_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_event_loop(monome: *mut monome_t);
}
extern "C" {
    pub fn monome_get_fd(monome: *mut monome_t) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " led grid commands"]
    pub fn monome_led_set(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
        on: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_on(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_off(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_all(
        monome: *mut monome_t,
        status: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_map(
        monome: *mut monome_t,
        x_off: ::std::os::raw::c_uint,
        y_off: ::std::os::raw::c_uint,
        data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_col(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y_off: ::std::os::raw::c_uint,
        count: usize,
        col_data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_row(
        monome: *mut monome_t,
        x_off: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
        count: usize,
        row_data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_intensity(
        monome: *mut monome_t,
        brightness: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_level_set(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
        level: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_level_all(
        monome: *mut monome_t,
        level: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_level_map(
        monome: *mut monome_t,
        x_off: ::std::os::raw::c_uint,
        y_off: ::std::os::raw::c_uint,
        data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_level_row(
        monome: *mut monome_t,
        x_off: ::std::os::raw::c_uint,
        y: ::std::os::raw::c_uint,
        count: usize,
        data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_level_col(
        monome: *mut monome_t,
        x: ::std::os::raw::c_uint,
        y_off: ::std::os::raw::c_uint,
        count: usize,
        data: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_event_get_grid(
        e: *const monome_event_t,
        out_x: *mut ::std::os::raw::c_uint,
        out_y: *mut ::std::os::raw::c_uint,
        monome: *mut *mut monome_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " led ring commands"]
    pub fn monome_led_ring_set(
        monome: *mut monome_t,
        ring: ::std::os::raw::c_uint,
        led: ::std::os::raw::c_uint,
        level: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_ring_all(
        monome: *mut monome_t,
        ring: ::std::os::raw::c_uint,
        level: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_ring_map(
        monome: *mut monome_t,
        ring: ::std::os::raw::c_uint,
        levels: *const u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_led_ring_range(
        monome: *mut monome_t,
        ring: ::std::os::raw::c_uint,
        start: ::std::os::raw::c_uint,
        end: ::std::os::raw::c_uint,
        level: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " tilt commands"]
    pub fn monome_tilt_enable(
        monome: *mut monome_t,
        sensor: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn monome_tilt_disable(
        monome: *mut monome_t,
        sensor: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
