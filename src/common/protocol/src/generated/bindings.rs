/* automatically generated by rust-bindgen 0.69.2 */

pub const BUFFER_SIZE: u32 = 64;
pub const START_SYMBOL: u8 = 35u8;
pub const END_SYMBOL: u8 = 69u8;
pub const MAGIC_NUMBER: u32 = 20;
pub const OUT_BUFFER_SIZE: u32 = 60;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SerialHal {
    pub data: *mut ::std::os::raw::c_void,
    pub inner_available: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub inner_send: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_uchar),
    >,
    pub inner_read: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_uchar,
    >,
    pub inner_flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}
#[test]
fn bindgen_test_layout_SerialHal() {
    const UNINIT: ::std::mem::MaybeUninit<SerialHal> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<SerialHal>(),
        40usize,
        concat!("Size of: ", stringify!(SerialHal))
    );
    assert_eq!(
        ::std::mem::align_of::<SerialHal>(),
        8usize,
        concat!("Alignment of ", stringify!(SerialHal))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SerialHal),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_available) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SerialHal),
            "::",
            stringify!(inner_available)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_send) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SerialHal),
            "::",
            stringify!(inner_send)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_read) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SerialHal),
            "::",
            stringify!(inner_read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_flush) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SerialHal),
            "::",
            stringify!(inner_flush)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal4initEv"]
    pub fn SerialHal_init(this: *mut SerialHal);
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal5flushEv"]
    pub fn SerialHal_flush(this: *mut SerialHal);
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal4sendEh"]
    pub fn SerialHal_send(this: *mut SerialHal, arg1: ::std::os::raw::c_uchar);
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal4readEv"]
    pub fn SerialHal_read(this: *mut SerialHal) -> ::std::os::raw::c_uchar;
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal9availableEv"]
    pub fn SerialHal_available(this: *mut SerialHal) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[link_name = "\u{1}_ZN9SerialHal4initEPvPFiS0_EPFvS0_hEPFhS0_EPFvS0_E"]
    pub fn SerialHal_init1(
        this: *mut SerialHal,
        data: *mut ::std::os::raw::c_void,
        inner_available: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        inner_send: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_uchar),
        >,
        inner_read: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_uchar,
        >,
        inner_flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    );
}
impl SerialHal {
    #[inline]
    pub unsafe fn init(&mut self) {
        SerialHal_init(self)
    }
    #[inline]
    pub unsafe fn flush(&mut self) {
        SerialHal_flush(self)
    }
    #[inline]
    pub unsafe fn send(&mut self, arg1: ::std::os::raw::c_uchar) {
        SerialHal_send(self, arg1)
    }
    #[inline]
    pub unsafe fn read(&mut self) -> ::std::os::raw::c_uchar {
        SerialHal_read(self)
    }
    #[inline]
    pub unsafe fn available(&mut self) -> ::std::os::raw::c_uint {
        SerialHal_available(self)
    }
    #[inline]
    pub unsafe fn init1(
        &mut self,
        data: *mut ::std::os::raw::c_void,
        inner_available: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
        inner_send: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_uchar),
        >,
        inner_read: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_uchar,
        >,
        inner_flush: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    ) {
        SerialHal_init1(
            self,
            data,
            inner_available,
            inner_send,
            inner_read,
            inner_flush,
        )
    }
}
extern "C" {
    #[link_name = "\u{1}_Z14new_serial_halv"]
    pub fn new_serial_hal() -> SerialHal;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Protocol {
    pub serial: SerialHal,
    pub buffer: [::std::os::raw::c_uchar; 64usize],
    pub pos: ::std::os::raw::c_uint,
    pub out_buffer: [::std::os::raw::c_uchar; 60usize],
    pub out_len: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Protocol() {
    const UNINIT: ::std::mem::MaybeUninit<Protocol> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Protocol>(),
        176usize,
        concat!("Size of: ", stringify!(Protocol))
    );
    assert_eq!(
        ::std::mem::align_of::<Protocol>(),
        8usize,
        concat!("Alignment of ", stringify!(Protocol))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).serial) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(serial)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pos) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).out_buffer) as usize - ptr as usize },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(out_buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).out_len) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(out_len)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8Protocol8send_msgEPhh"]
    pub fn Protocol_send_msg(
        this: *mut Protocol,
        buff: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8Protocol16try_read_messageEv"]
    pub fn Protocol_try_read_message(this: *mut Protocol) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN8Protocol4initE9SerialHal"]
    pub fn Protocol_init(this: *mut Protocol, serial: SerialHal);
}
impl Protocol {
    #[inline]
    pub unsafe fn send_msg(
        &mut self,
        buff: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uchar,
    ) {
        Protocol_send_msg(self, buff, len)
    }
    #[inline]
    pub unsafe fn try_read_message(&mut self) -> bool {
        Protocol_try_read_message(self)
    }
    #[inline]
    pub unsafe fn init(&mut self, serial: SerialHal) {
        Protocol_init(self, serial)
    }
}
extern "C" {
    #[link_name = "\u{1}_Z12new_protocolv"]
    pub fn new_protocol() -> Protocol;
}
