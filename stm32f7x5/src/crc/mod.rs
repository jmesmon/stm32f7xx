#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data register"]
    pub dr: DR,
    #[doc = "0x04 - Independent Data register"]
    pub idr: IDR,
    #[doc = "0x08 - Control register"]
    pub cr: CR,
    #[doc = "0x0c - Initial CRC value"]
    pub init: INIT,
    #[doc = "0x10 - CRC polynomial"]
    pub pol: POL,
}
#[doc = "Data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data register"]
pub mod dr;
#[doc = "Independent Data register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Independent Data register"]
pub mod idr;
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "Initial CRC value"]
pub struct INIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Initial CRC value"]
pub mod init;
#[doc = "CRC polynomial"]
pub struct POL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC polynomial"]
pub mod pol;
