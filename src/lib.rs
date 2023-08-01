#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(ambiguous_glob_reexports)]

pub mod bindings {
    #[path = "R.rs"]
    pub mod r;

    #[path = "Rinternals.rs"]
    pub mod r_internals {
        use super::r_ext::boolean::Rboolean;
        use super::r_ext::complex::Rcomplex;
        use super::r_ext::r_dynload::DL_FUNC;

        include!("bindings/Rinternals.rs");
    }

    #[path = "Rmath.rs"]
    pub mod r_math;

    #[path = "Rversion.rs"]
    pub mod r_version;

    //TODO: unix specific?
    // #[path = "Rinterface.rs"]
    // pub mod r_interface;
    // pub use r_interface::*;

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

        // #[path = "Callbacks.rs"]
        pub mod callbacks {
            use super::super::r_internals::SEXP;
            use super::boolean::Rboolean;

            include!("bindings/R_ext/Callbacks.rs");
        }

        //TODO: another platform?
        // #[path = "GetX11Image.rs"]
        // pub mod GetX11Image;

        // #[path ="Lapack.rs"]
        pub mod lapack {
            use super::complex::Rcomplex;
            include!("bindings/R_ext/Lapack.rs");
        }

        #[path = "Linpack.rs"]
        pub mod linpack;

        #[path = "Parse.rs"]
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

        // TODO: another platform?
        // #[path ="eventloop.rs"]
        // pub mod event_loop;

        #[path = "Boolean.rs"]
        pub mod boolean;

        #[path = "Complex.rs"]
        pub mod complex;

        #[path = "Arith.rs"]
        pub mod arith;

        #[path = "Constants.rs"]
        pub mod constants;

        #[path = "Error.rs"]
        pub mod error;

        #[path = "Memory.rs"]
        pub mod memory;

        #[path = "Print.rs"]
        pub mod print;

        #[path = "RS.rs"]
        pub mod rs;

        #[path = "Random.rs"]
        pub mod random;

        #[path = "Utils.rs"]
        pub mod utils {
            use super::complex::Rcomplex;
            use super::boolean::Rboolean;

            include!("bindings/R_ext/Utils.rs");
        }

        //TODO: this is windows specific.
        #[path = "libextern.rs"]
        pub mod libextern;
    }

    #[path = "Rconfig.rs"]
    pub mod r_config;

    pub mod r_prelude {
        pub use super::r_config::*;
        pub use super::r_ext::arith::*;
        pub use super::r_ext::boolean::*;
        pub use super::r_ext::complex::*;
        pub use super::r_ext::constants::*;
        pub use super::r_ext::error::*;
        pub use super::r_ext::libextern::*;
        pub use super::r_ext::memory::*;
        pub use super::r_ext::print::*;
        pub use super::r_ext::random::*;
        pub use super::r_ext::rs::*;
        pub use super::r_ext::utils::*;
    }
}