/* automatically generated by rust-bindgen 0.69.2 */

pub const BUFFER_SIZE: u32 = 200;
pub const START_SYMBOL: u8 = 35u8;
pub const END_SYMBOL: u8 = 69u8;
pub const MAGIC_NUMBER: u32 = 20;
pub const OUT_BUFFER_SIZE: u32 = 200;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Checker {
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
    pub buffer: [::std::os::raw::c_uchar; 200usize],
    pub pos: ::std::os::raw::c_uint,
    pub out_buffer: [::std::os::raw::c_uchar; 200usize],
}
#[test]
fn bindgen_test_layout_Checker() {
    const UNINIT: ::std::mem::MaybeUninit<Checker> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Checker>(),
        448usize,
        concat!("Size of: ", stringify!(Checker))
    );
    assert_eq!(
        ::std::mem::align_of::<Checker>(),
        8usize,
        concat!("Alignment of ", stringify!(Checker))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_available) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(inner_available)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_send) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(inner_send)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_read) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(inner_read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner_flush) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(inner_flush)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pos) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).out_buffer) as usize - ptr as usize },
        244usize,
        concat!(
            "Offset of field: ",
            stringify!(Checker),
            "::",
            stringify!(out_buffer)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN7Checker8send_msgEPhh"]
    pub fn Checker_send_msg(
        this: *mut Checker,
        buff: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uchar,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN7Checker16try_read_messageEv"]
    pub fn Checker_try_read_message(this: *mut Checker) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZN7Checker4initEPvPFiS0_EPFvS0_hEPFhS0_EPFvS0_E"]
    pub fn Checker_init(
        this: *mut Checker,
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
impl Checker {
    #[inline]
    pub unsafe fn send_msg(
        &mut self,
        buff: *mut ::std::os::raw::c_uchar,
        len: ::std::os::raw::c_uchar,
    ) {
        Checker_send_msg(self, buff, len)
    }
    #[inline]
    pub unsafe fn try_read_message(&mut self) -> bool {
        Checker_try_read_message(self)
    }
    #[inline]
    pub unsafe fn init(
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
        Checker_init(
            self,
            data,
            inner_available,
            inner_send,
            inner_read,
            inner_flush,
        )
    }
}
pub const msg_type_Ok: msg_type = 0;
pub const msg_type_SpawnWall: msg_type = 1;
pub const msg_type_Joystick: msg_type = 2;
pub type msg_type = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Protocol {
    pub checker: Checker,
}
#[test]
fn bindgen_test_layout_Protocol() {
    const UNINIT: ::std::mem::MaybeUninit<Protocol> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Protocol>(),
        448usize,
        concat!("Size of: ", stringify!(Protocol))
    );
    assert_eq!(
        ::std::mem::align_of::<Protocol>(),
        8usize,
        concat!("Alignment of ", stringify!(Protocol))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).checker) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Protocol),
            "::",
            stringify!(checker)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN8Protocol4initEPvPFiS0_EPFvS0_hEPFhS0_EPFvS0_E"]
    pub fn Protocol_init(
        this: *mut Protocol,
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
impl Protocol {
    #[inline]
    pub unsafe fn init(
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
        Protocol_init(
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
    #[link_name = "\u{1}_Z12new_protocolv"]
    pub fn new_protocol() -> Protocol;
}
