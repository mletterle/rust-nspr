use std::libc::{c_char, c_int, c_void, c_uint, c_ushort};

#[link_args = "-lnspr4"]
#[nolink]

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
extern "C" {  
fn PR_OpenTCPSocket(af: c_ushort) -> *c_void;
fn PR_Connect(fd: *c_void, addr: *PRNetAddr, timout: c_uint) -> PRStatus;
fn PR_Close(fd: *c_void) -> PRStatus;
fn PR_StringToNetAddr(string: *c_char, addr: *c_void) -> PRStatus;
fn PR_GetError() -> c_int;
fn PR_ErrorToName(error: c_int) -> *c_char;
fn PR_htons(conversion: c_ushort) -> c_ushort;
fn PR_htonl(conversion: c_uint) -> c_uint;
fn PR_Write(fd: *c_void, buf: *c_void, amount: c_int) -> c_int;
fn PR_Read(fd: *c_void, buf: *c_void, amount: c_int) -> c_int;
}
