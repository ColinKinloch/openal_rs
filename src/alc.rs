/* automatically generated by rust-bindgen */

pub const ALC_INVALID: ::std::os::raw::c_uint = 0;
pub const ALC_VERSION_0_1: ::std::os::raw::c_uint = 1;
pub const ALC_FALSE: ::std::os::raw::c_uint = 0;
pub const ALC_TRUE: ::std::os::raw::c_uint = 1;
pub const ALC_FREQUENCY: ::std::os::raw::c_uint = 4103;
pub const ALC_REFRESH: ::std::os::raw::c_uint = 4104;
pub const ALC_SYNC: ::std::os::raw::c_uint = 4105;
pub const ALC_MONO_SOURCES: ::std::os::raw::c_uint = 4112;
pub const ALC_STEREO_SOURCES: ::std::os::raw::c_uint = 4113;
pub const ALC_NO_ERROR: ::std::os::raw::c_uint = 0;
pub const ALC_INVALID_DEVICE: ::std::os::raw::c_uint = 40961;
pub const ALC_INVALID_CONTEXT: ::std::os::raw::c_uint = 40962;
pub const ALC_INVALID_ENUM: ::std::os::raw::c_uint = 40963;
pub const ALC_INVALID_VALUE: ::std::os::raw::c_uint = 40964;
pub const ALC_OUT_OF_MEMORY: ::std::os::raw::c_uint = 40965;
pub const ALC_MAJOR_VERSION: ::std::os::raw::c_uint = 4096;
pub const ALC_MINOR_VERSION: ::std::os::raw::c_uint = 4097;
pub const ALC_ATTRIBUTES_SIZE: ::std::os::raw::c_uint = 4098;
pub const ALC_ALL_ATTRIBUTES: ::std::os::raw::c_uint = 4099;
pub const ALC_DEFAULT_DEVICE_SPECIFIER: ::std::os::raw::c_uint = 4100;
pub const ALC_DEVICE_SPECIFIER: ::std::os::raw::c_uint = 4101;
pub const ALC_EXTENSIONS: ::std::os::raw::c_uint = 4102;
pub const ALC_EXT_CAPTURE: ::std::os::raw::c_uint = 1;
pub const ALC_CAPTURE_DEVICE_SPECIFIER: ::std::os::raw::c_uint = 784;
pub const ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER: ::std::os::raw::c_uint = 785;
pub const ALC_CAPTURE_SAMPLES: ::std::os::raw::c_uint = 786;
pub const ALC_ENUMERATE_ALL_EXT: ::std::os::raw::c_uint = 1;
pub const ALC_DEFAULT_ALL_DEVICES_SPECIFIER: ::std::os::raw::c_uint = 4114;
pub const ALC_ALL_DEVICES_SPECIFIER: ::std::os::raw::c_uint = 4115;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCdevice_struct {
    _unused: [u8; 0],
}
/** Opaque device handle */
pub type ALCdevice = ALCdevice_struct;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ALCcontext_struct {
    _unused: [u8; 0],
}
/** Opaque context handle */
pub type ALCcontext = ALCcontext_struct;
/** 8-bit boolean */
pub type ALCboolean = ::std::os::raw::c_char;
/** character */
pub type ALCchar = ::std::os::raw::c_char;
/** signed 8-bit 2's complement integer */
pub type ALCbyte = ::std::os::raw::c_schar;
/** unsigned 8-bit integer */
pub type ALCubyte = ::std::os::raw::c_uchar;
/** signed 16-bit 2's complement integer */
pub type ALCshort = ::std::os::raw::c_short;
/** unsigned 16-bit integer */
pub type ALCushort = ::std::os::raw::c_ushort;
/** signed 32-bit 2's complement integer */
pub type ALCint = ::std::os::raw::c_int;
/** unsigned 32-bit integer */
pub type ALCuint = ::std::os::raw::c_uint;
/** non-negative 32-bit binary integer size */
pub type ALCsizei = ::std::os::raw::c_int;
/** enumerated 32-bit value */
pub type ALCenum = ::std::os::raw::c_int;
/** 32-bit IEEE754 floating-point */
pub type ALCfloat = f32;
/** 64-bit IEEE754 floating-point */
pub type ALCdouble = f64;
/** void type (for opaque pointers only) */
pub type ALCvoid = ::std::os::raw::c_void;
extern "C" {
    /** Context management. */
    pub fn alcCreateContext(device: *mut ALCdevice, attrlist: *const ALCint)
     -> *mut ALCcontext;
}
extern "C" {
    pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
}
extern "C" {
    pub fn alcProcessContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcSuspendContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcDestroyContext(context: *mut ALCcontext);
}
extern "C" {
    pub fn alcGetCurrentContext() -> *mut ALCcontext;
}
extern "C" {
    pub fn alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice;
}
extern "C" {
    /** Device management. */
    pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
}
extern "C" {
    pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
extern "C" {
    /**
 * Error support.
 *
 * Obtain the most recent Device error.
 */
    pub fn alcGetError(device: *mut ALCdevice) -> ALCenum;
}
extern "C" {
    /**
 * Extension support.
 *
 * Query for the presence of an extension, and obtain any appropriate
 * function pointers and enum values.
 */
    pub fn alcIsExtensionPresent(device: *mut ALCdevice,
                                 extname: *const ALCchar) -> ALCboolean;
}
extern "C" {
    pub fn alcGetProcAddress(device: *mut ALCdevice, funcname: *const ALCchar)
     -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar)
     -> ALCenum;
}
extern "C" {
    /** Query function. */
    pub fn alcGetString(device: *mut ALCdevice, param: ALCenum)
     -> *const ALCchar;
}
extern "C" {
    pub fn alcGetIntegerv(device: *mut ALCdevice, param: ALCenum,
                          size: ALCsizei, values: *mut ALCint);
}
extern "C" {
    /** Capture function. */
    pub fn alcCaptureOpenDevice(devicename: *const ALCchar,
                                frequency: ALCuint, format: ALCenum,
                                buffersize: ALCsizei) -> *mut ALCdevice;
}
extern "C" {
    pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
}
extern "C" {
    pub fn alcCaptureStart(device: *mut ALCdevice);
}
extern "C" {
    pub fn alcCaptureStop(device: *mut ALCdevice);
}
extern "C" {
    pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid,
                             samples: ALCsizei);
}
/** Pointer-to-function type, useful for dynamically getting ALC entry points. */
pub type LPALCCREATECONTEXT =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               attrlist: *const ALCint)
                              -> *mut ALCcontext>;
pub type LPALCMAKECONTEXTCURRENT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)
                              -> ALCboolean>;
pub type LPALCPROCESSCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCSUSPENDCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCDESTROYCONTEXT =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCGETCURRENTCONTEXT =
    ::std::option::Option<unsafe extern "C" fn() -> *mut ALCcontext>;
pub type LPALCGETCONTEXTSDEVICE =
    ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)
                              -> *mut ALCdevice>;
pub type LPALCOPENDEVICE =
    ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar)
                              -> *mut ALCdevice>;
pub type LPALCCLOSEDEVICE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCboolean>;
pub type LPALCGETERROR =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCenum>;
pub type LPALCISEXTENSIONPRESENT =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               extname: *const ALCchar)
                              -> ALCboolean>;
pub type LPALCGETPROCADDRESS =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               funcname: *const ALCchar)
                              -> *mut ::std::os::raw::c_void>;
pub type LPALCGETENUMVALUE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               enumname: *const ALCchar)
                              -> ALCenum>;
pub type LPALCGETSTRING =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               param: ALCenum)
                              -> *const ALCchar>;
pub type LPALCGETINTEGERV =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               param: ALCenum, size: ALCsizei,
                                               values: *mut ALCint)>;
pub type LPALCCAPTUREOPENDEVICE =
    ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar,
                                               frequency: ALCuint,
                                               format: ALCenum,
                                               buffersize: ALCsizei)
                              -> *mut ALCdevice>;
pub type LPALCCAPTURECLOSEDEVICE =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)
                              -> ALCboolean>;
pub type LPALCCAPTURESTART =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESTOP =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESAMPLES =
    ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice,
                                               buffer: *mut ALCvoid,
                                               samples: ALCsizei)>;
