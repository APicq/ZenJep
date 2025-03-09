pub mod aircrafts;
pub mod families;
pub mod filter;
pub mod home;
pub mod jeppesen;
pub mod years;

pub mod allpages {
    pub use super::aircrafts::page_aircrafts;
    pub use super::families::page_families;
    pub use super::filter::page_filter;
    pub use super::home::page_home;
    pub use super::jeppesen::page_jeppesen;
    pub use super::years::page_years;
}
