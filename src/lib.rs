pub mod manifest {
    pub mod domain {
        pub mod model {
            pub mod entity;
            pub mod lang;
            pub mod arch;
            pub mod root;
        }
        pub mod repository;
    }
    pub mod infra {
        pub mod repository;
    }
    pub mod usecase {
        pub mod load {
            pub mod manifest;
        }
        pub mod generate {
            pub mod codefile;
        }
    }
    pub mod template;
}
pub mod codefile {
    pub mod template {
        pub mod utils;
    }
}
pub mod rust {
    pub mod management {
        pub mod project {
            pub mod wide;
            pub mod narrow;
        }
    }
} // Automatically exported by saba.
