#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key0: Key0,
    _reserved1: [u8; 0x7c],
    key1: Key1,
    _reserved2: [u8; 0x7c],
    key2: Key2,
    _reserved3: [u8; 0x7c],
    key3: Key3,
}
impl RegisterBlock {
    #[doc = "0x00 - AES Key 0."]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x80 - AES Key 1."]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x100 - AES Key 2."]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x180 - AES Key 3."]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
}
#[doc = "KEY0 (rw) register accessor: AES Key 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`] module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "AES Key 0."]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: AES Key 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`] module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "AES Key 1."]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: AES Key 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`] module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "AES Key 2."]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: AES Key 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`] module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "AES Key 3."]
pub mod key3;
