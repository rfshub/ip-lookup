/* src/api.rs */

use crate::lookup::{
    lookup_with_ipapi,
    lookup_with_ipinfo,
    lookup_with_ipsb,
    lookup_with_ipapiio,
    lookup_with_apipcc,
    lookup_with_ipapiis,
    lookup_with_geolocated,
    lookup_with_iplocationapi,
    LookupResult,
};

#[derive(Debug, Copy, Clone)]
pub enum LookupProvider {
    IpApi,
    IpInfo,
    IpSb,
    IpApiIo,
    ApipCc,
    IpapiIs,
    Geolocated,
    IpLocationApi,
}

pub fn lookup(provider: LookupProvider) -> Option<LookupResult> {
    match provider {
        LookupProvider::IpApi => lookup_with_ipapi(),
        LookupProvider::IpInfo => lookup_with_ipinfo(),
        LookupProvider::IpSb => lookup_with_ipsb(),
        LookupProvider::IpApiIo => lookup_with_ipapiio(),
        LookupProvider::ApipCc => lookup_with_apipcc(),
        LookupProvider::IpapiIs => lookup_with_ipapiis(),
        LookupProvider::Geolocated => lookup_with_geolocated(),
        LookupProvider::IpLocationApi => lookup_with_iplocationapi(),
    }
}

impl LookupProvider {
    pub fn all() -> &'static [LookupProvider] {
        &[
            LookupProvider::IpApi,
            LookupProvider::IpInfo,
            LookupProvider::IpSb,
            LookupProvider::IpApiIo,
            LookupProvider::ApipCc,
            LookupProvider::IpapiIs,
            LookupProvider::Geolocated,
            LookupProvider::IpLocationApi,
        ]
    }
}
