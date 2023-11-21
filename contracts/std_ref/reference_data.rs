use crate::constant::E18;
use crate::ref_data::RefDatum;
use crate::std_ref::Error;

#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ReferenceData {
    pub rate: u128,
    pub base_resolve_time: u64,
    pub quote_resolve_time: u64,
}

impl ReferenceData {
    pub fn new(rate: u128, base_resolve_time: u64, quote_resolve_time: u64) -> Self {
        Self {
            rate,
            base_resolve_time,
            quote_resolve_time,
        }
    }

    pub fn from_ref_data_pair(base: RefDatum, quote: RefDatum) -> Result<Self, Error> {
        Ok(Self {
            rate: (base.rate as u128)
                .checked_mul(E18)
                .ok_or(Error::InvalidValue)?
                .checked_div(quote.rate as u128)
                .ok_or(Error::InvalidValue)?,
            base_resolve_time: base.resolve_time,
            quote_resolve_time: quote.resolve_time,
        })
    }
}
