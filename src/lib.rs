use anchor_lang::prelude::*;

#[repr(C)]
#[zero_copy(unsafe)]
#[repr(packed)]
#[derive(Default, Debug, PartialEq)]
pub struct TickSqrtPricInfo {
    pub sqrt_price: [f64; 2],
}

#[repr(C)]
#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct TickSqrtPriceMap {
    pub tick_sqrt_price_map: [TickSqrtPricInfo; 443636],
}

impl Default for TickSqrtPriceMap {
    fn default() -> Self {
        TickSqrtPriceMap {
            tick_sqrt_price_map: [TickSqrtPricInfo::default(); 443636],
        }
    }
}


anchor_lang::declare_id!("3sCp3TbbHjoo1RKPGxkGv1msDRn5giFSEBjDUvNgZotD");
