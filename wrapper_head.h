#define R_NO_REMAP
#define R_NO_REMAP_RMATH
#define STRICT_R_HEADERS

// From r83513 (R 4.3), R defines the `NORET` macro differently depending on the
// C/C++ standard the compiler uses. It matters when the header is used in C/C++
// libraries, but all we want to do here is to make bindgen interpret `NOREP` to
// `!`. However, for some reason, bindgen doesn't handle other no-return
// attributes like `_Noreturn` (for C11) and `[[noreturn]]` (for C++ and C23),
// so we define it here.
#define NORET __attribute__((__noreturn__))

#include <stddef.h> // for ptrdiff_t

// // R_xlen_t is defined as int on 32-bit platforms, and
// // that confuses Rust. Keeping it always as ptrdiff_t works
// // fine even on 32-bit.
// /// <div rustbindgen replaces="R_xlen_t"></div>
// typedef ptrdiff_t R_xlen_t_rust;

#ifdef SWITCH_TO_REFCOUNT
#define SWITCH_TO_REFCOUNT_RUST_MACROS
#else
#define SWITCH_TO_NAMED_RUST_MACROS
#endif
