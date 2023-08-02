// #![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub mod bindings {
    #[path = "R.rs"]
    #[allow(improper_ctypes)] // unix
    pub mod r;

    #[path = "Rinternals.rs"]
    #[allow(improper_ctypes)] // unix
    pub mod r_internals {
        use super::r_ext::boolean::Rboolean;
        use super::r_ext::complex::Rcomplex;
        use super::r_ext::r_dynload::DL_FUNC;

        include!("bindings/Rinternals.rs");
    }

    #[path = "Rmath.rs"]
    #[allow(improper_ctypes)] // unix
    pub mod r_math;

    #[path = "Rversion.rs"]
    pub mod r_version;

    //TODO: unix specific?
    #[cfg(unix)]
    pub mod r_interface {
        use super::r_ext::boolean::Rboolean;
        include!("bindings/Rinterface.rs");
    }

    pub mod r_embedded {
        use super::r_ext::boolean::Rboolean;

        include!("bindings/Rembedded.rs");
    }

    #[path = "R_ext"]
    pub mod r_ext {
        pub mod applic {
            use super::boolean::Rboolean;
            include!("bindings/R_ext/Applic.rs");
        }

        pub mod blas {
            use super::complex::Rcomplex;
            include!("bindings/R_ext/BLAS.rs");
        }

        #[allow(improper_ctypes)] // unix
        pub mod callbacks {
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;

            include!("bindings/R_ext/Callbacks.rs");
        }

        //TODO: another platform?
        #[cfg(unix)]
        pub mod get_x11_image {
            use super::boolean::Rboolean;
            include!("bindings/R_ext/GetX11Image.rs");
        }

        pub mod lapack {
            use super::complex::Rcomplex;
            include!("bindings/R_ext/Lapack.rs");
        }

        #[path = "Linpack.rs"]
        pub mod linpack;

        #[path = "Parse.rs"]
        #[allow(improper_ctypes)] // unix
        pub mod parse {
            use super::super::r_internals::SEXP;
            include!("bindings/R_ext/Parse.rs");
        }

        pub mod r_startup {
            use super::boolean::Rboolean;
            include!("bindings/R_ext/RStartup.rs");
        }

        pub mod r_dynload {
            use super::boolean::Rboolean;
            include!("bindings/R_ext/Rdynload.rs");
        }

        #[path = "Riconv.rs"]
        pub mod r_iconv;

        #[path = "Visibility.rs"]
        pub mod visibility;

        
        #[path = "eventloop.rs"]
        #[cfg(unix)]
        pub mod event_loop;

        #[path = "Boolean.rs"]
        pub mod boolean;

        #[path = "Complex.rs"]
        pub mod complex;

        #[path = "Arith.rs"]
        #[allow(improper_ctypes)] // unix
        pub mod arith;

        #[path = "Constants.rs"]
        pub mod constants;

        #[path = "Error.rs"]
        pub mod error;

        #[path = "Memory.rs"]
        #[allow(improper_ctypes)] // unix
        pub mod memory;

        #[path = "Print.rs"]
        pub mod print;

        #[path = "RS.rs"]
        pub mod rs;

        #[path = "Random.rs"]
        pub mod random;

        #[path = "Utils.rs"]
        pub mod utils {
            use super::boolean::Rboolean;
            use super::complex::Rcomplex;

            include!("bindings/R_ext/Utils.rs");
        }

        #[path = "libextern.rs"]
        #[cfg(windows)]
        pub mod libextern;

        // region: unmentioned api

        #[path = "QuartzDevice.rs"]
        pub mod quartz_device;

        #[allow(improper_ctypes)] // unix
        pub mod graphics_device {
            #[cfg(windows)]
            use super::super::r_internals::cetype_t;
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;
            #[cfg(unix)]
            use super::graphics_engine::pGEcontext;

            include!("bindings/R_ext/GraphicsDevice.rs");
        }

        #[allow(improper_ctypes)] // unix
        pub mod graphics_engine {
            use super::super::r_internals::cetype_t;
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;
            use super::graphics_device::pDevDesc;

            include!("bindings/R_ext/GraphicsEngine.rs");
        }

        #[allow(improper_ctypes)] // unix
        pub mod connections {
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;
            include!("bindings/R_ext/Connections.rs");
        }
        #[path = "Itermacros.rs"]
        pub mod itermacros;

        #[path = "MathThreads.rs"]
        pub mod math_threads;

        #[allow(improper_ctypes)] // unix
        pub mod prt_util {
            use super::super::r_internals::{R_xlen_t, SEXP};
            use super::complex::Rcomplex;

            include!("bindings/R_ext/PrtUtil.rs");
        }

        #[path = "Rallocators.rs"]
        pub mod r_allocators;

        pub mod stats_package;

        #[allow(improper_ctypes)] // unix
        pub mod stats_stubs;

        // endregion
    }

    // region: unmentioned API
    #[cfg(windows)]
    pub mod ga {
        use super::graphapp::*;
        include!("bindings/ga.rs");
    }

    #[cfg(windows)]
    #[path = "graphapp.rs"]
    pub mod graphapp;

    #[path = "iconv.rs"]
    #[cfg(windows)]
    pub mod iconv;
    
    #[cfg(windows)]
    pub mod libintl;

    // TODO: ignore this... or make a warning
    #[allow(improper_ctypes)] // unix
    pub mod r_defines {
        use super::r_internals::SEXPREC;
        include!("bindings/Rdefines.rs");
    }

    // endregion

    #[path = "Rconfig.rs"]
    pub mod r_config;

    pub mod r_prelude {
        #![allow(ambiguous_glob_reexports)]
        //! The `R.h`-header would provide all of these objects, plus itself.
        //!
        pub use super::r_config::*;
        pub use super::r_ext::arith::*;
        pub use super::r_ext::boolean::*;
        pub use super::r_ext::complex::*;
        pub use super::r_ext::constants::*;
        pub use super::r_ext::error::*;
        #[cfg(windows)]
        pub use super::r_ext::libextern::*;
        pub use super::r_ext::memory::*;
        pub use super::r_ext::print::*;
        pub use super::r_ext::random::*;
        pub use super::r_ext::rs::*;
        pub use super::r_ext::utils::*;

        // note: should this also be included? I guess so?
        pub use super::r::*;
    }
}
