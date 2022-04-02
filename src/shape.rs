// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::os::raw::{ c_int };

use crate::xlib::{ Display, XID, XRectangle };


//
// functions
//


x11_link! { Xss, shape, ["libXss.so.2", "libXss.so"], 1,
  pub fn XShapeCombineRectangles (_1: *mut Display, _2: XID, _3: c_int, _4: c_int, _5: c_int, _6: *mut XRectangle, _7: c_int, _8: c_int, _9: c_int) -> (),
variadic:
globals:
}

