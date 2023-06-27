/* automatically generated by rust-bindgen 0.66.1 */

pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
pub type tailscale = ::std::os::raw::c_int;
extern "C" {
    pub fn tailscale_new() -> tailscale;
}
extern "C" {
    pub fn tailscale_start(sd: tailscale) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_up(sd: tailscale) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_close(sd: tailscale) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_dir(
        sd: tailscale,
        dir: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_hostname(
        sd: tailscale,
        hostname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_authkey(
        sd: tailscale,
        authkey: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_control_url(
        sd: tailscale,
        control_url: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_ephemeral(
        sd: tailscale,
        ephemeral: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_set_logfd(sd: tailscale, fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub type tailscale_conn = ::std::os::raw::c_int;
extern "C" {
    pub fn tailscale_dial(
        sd: tailscale,
        network: *const ::std::os::raw::c_char,
        addr: *const ::std::os::raw::c_char,
        conn_out: *mut tailscale_conn,
    ) -> ::std::os::raw::c_int;
}
pub type tailscale_listener = ::std::os::raw::c_int;
extern "C" {
    pub fn tailscale_listen(
        sd: tailscale,
        network: *const ::std::os::raw::c_char,
        addr: *const ::std::os::raw::c_char,
        listener_out: *mut tailscale_listener,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_accept(
        listener: tailscale_listener,
        conn_out: *mut tailscale_conn,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_loopback(
        sd: tailscale,
        addr_out: *mut ::std::os::raw::c_char,
        addrlen: usize,
        proxy_cred_out: *mut ::std::os::raw::c_char,
        local_api_cred_out: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tailscale_errmsg(
        sd: tailscale,
        buf: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> ::std::os::raw::c_int;
}
