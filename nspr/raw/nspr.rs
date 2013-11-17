use std::libc::{c_char, c_int, c_void, c_uint, c_ushort};


pub type PRBool = c_int;
pub type PRStatus = c_int;

pub static PRTrue: PRBool = 1;
pub static PRFalse: PRBool = 0;

pub static PRSuccess: PRStatus = 0;
pub static PRFailure: PRStatus = -1;

pub static PR_AF_INET: c_ushort = 2;

pub struct PRNetAddr {
     family: c_ushort,
     port: c_ushort,
     ip: c_uint,
     pad: [c_char, ..8],

}

#[link_args = "-lnspr4"]
extern "C" {  
pub fn PR_OpenTCPSocket(af: c_ushort) -> *c_void;
pub fn PR_Connect(fd: *c_void, addr: *PRNetAddr, timout: c_uint) -> PRStatus;
pub fn PR_Close(fd: *c_void) -> PRStatus;
pub fn PR_StringToNetAddr(string: *c_char, addr: *c_void) -> PRStatus;
pub fn PR_GetError() -> c_int;
pub fn PR_ErrorToName(error: c_int) -> *c_char;
pub fn PR_htons(conversion: c_ushort) -> c_ushort;
pub fn PR_htonl(conversion: c_uint) -> c_uint;
pub fn PR_Write(fd: *c_void, buf: *c_void, amount: c_int) -> c_int;
pub fn PR_Read(fd: *c_void, buf: *c_void, amount: c_int) -> c_int;
}
