pub mod config;
pub mod consts;
pub mod deadlock;
pub mod handler;
pub mod re;
pub mod types;

pub mod events {
    pub mod disconnect;
    pub mod init;
    pub mod login;
    pub mod spawn;
    pub mod tick;

    pub mod packet {
        pub mod any;
        pub mod parser;
    }

    pub mod chat {
        pub mod any;
        pub mod clan;
        pub mod global;
        pub mod local;
        pub mod parser;
        pub mod personal;
        pub mod unknown;
    }
}
