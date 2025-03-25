#[cfg(test)]
mod tests {
    use crate::soap_envelope::{
        crypto::{sha256_and_sign_with_pfx, sha256_base64, to_base64},
        soap_envelope::SoapEnvelope,
    };

    #[test]
    fn test_sha_base64() {
        let data = [1u8, 2u8, 3u8, 4u8, 5u8];
        let hash = sha256_base64(&data);
        let expected_hash = "dPgf4WfZm0y0HW0MzagieMrunz4vJdXlo5Nv89zsYNA=";
        assert_eq!(hash, expected_hash)
    }

    #[test]
    fn test_signed_soapenv_with_timestamp() {
        todo!();
    }

    #[test]
    fn test_signed_soapenv() {
        // SoapEnvelope::
        todo!();
    }

    const CERT_PUBLIC: &str = "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAz8q1sR3OERJXHkX0dJJgiQUJK07G2/48MeIBXVeHd49jEmf7SAP4/S00EGspMhTFQDlZ2QkhtiBBSmQMjAcGm4vwz+uxR05+jeMhmcTxO5IVf+gnr1spd0udLNq30hwfJk2qlTOO+Oc0LZQA8eO6pvlZl9rGkFC2HPhCTsNPglWFRnErMn7YkRF7Rptk5ra4/+1RxuUjiGkfMNuDhxV00Gf8Y5BX4eoVacAw6pInfjZsQO+vbXO5Z+7kGWDfC06PcRWfWYOMmnhIs6tQDqIGM/j85NjrTGXZvvK9PNQGK1cz/PdyoTn5lFGT8ZW0/mrBHNKC0jgI5RkaxwxzKUKbbQIDAQAB";
    const CERT_PRIV: &str = "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDPyrWxHc4RElceRfR0kmCJBQkrTsbb/jwx4gFdV4d3j2MSZ/tIA/j9LTQQaykyFMVAOVnZCSG2IEFKZAyMBwabi/DP67FHTn6N4yGZxPE7khV/6CevWyl3S50s2rfSHB8mTaqVM4745zQtlADx47qm+VmX2saQULYc+EJOw0+CVYVGcSsyftiREXtGm2Tmtrj/7VHG5SOIaR8w24OHFXTQZ/xjkFfh6hVpwDDqkid+NmxA769tc7ln7uQZYN8LTo9xFZ9Zg4yaeEizq1AOogYz+Pzk2OtMZdm+8r081AYrVzP893KhOfmUUZPxlbT+asEc0oLSOAjlGRrHDHMpQpttAgMBAAECggEAKKtZMmhN+8NmL7Ora+F2aXsF12ccvtQcvfqpH7bQ+dKjpmeZo/e7FPpy9T+0GWw9SxuufS8vXPElNkUsu39oiKs0H83WrcksNeMdoXYNbQZjlNxAYC7sh7/R7ISGc+YzJpEO8RLdSdQev0j3gmB7GNE2+uTD9l0Ft9fTSo0pk62EvLXZ0WyvkoRXnGG5baRLEiPO6soQFt7vbWQQ1ertzn3KJ6+f5sbizJsmPs/e7or4SIjv8v+arhuxxjBSJ3/c++4PAf/flEzL7eINj7A/En9xut1OkBoOZqAmWsfptKjrN1xAbKYDJBLtrRewedDxXzVic8gNpZ1xmIN1K71ANQKBgQDTKzKJ4a1owNwKi5oOVNVyszLpBxrX+tsoJ97AoLK+D2Czemt+SIrsygXQ+JLtfm488/C4hTCKSVuVpUm42834OTkIm5MSi2rcYdhZ8QfBvx3va/P5ArrYppcYp4BSUhZ2ntFzzmxhxL/VECBIvTeTmD0CBhTTiaDeKFDBluq5vwKBgQD75/skVXVPqDb+mMVO/500NY84GY+uYsOqbh/IbbmOZAhenYw9evrmDfTf1hfwhp6YkZEgYY42VHlnSI9y09XXWNnqbJ0AVQRv/47WEYMPpyjDnzFbUWtUGz2qocZ9nJGDBnbKDodD/GS0pFIBlu5CaSqs3HP4MWWIM5QA8Lc90wKBgGecXmPA03D+j/isnp5BiamJu4US81zdvQJq7aTeNFWE/hGSE4QW2/Nq/IeYL59P1Y8ashYXY8W2ULWQMCf/0YPlr9JFY1hKB9iyOZGH7iJmP63l7gNUD5GVy7VRGmlJ1bPGIUcNFaiy/Nzx2KVYEhjdLbH8geN5N/FJHrad8fXJAoGBAN7JPxLWRccqZWDr6ezBIt5u0/hwmuNG0/fiJ1fSuv4UuFY9ji89mbJm+4APT+LYnGEgtLJntSeVtD1FLiEG+qPXH/s1DfGiPydyZHgsyrXIR8QjAbramkqrQPGs2+hx1TuFNv/is3zMNqCQhzqCqruvWR/CZQpHXZ2EyEvAmL8jAoGAQf7HM5be+z2a64GvoOTtHp0UrS40V3ba7RqiLvCVXGcA0KHMiJdsdXgq/QjcQl7puZrGOpi+3RRzCvzXgkyfHzKAyLEk2ynG3vVTxz7JN6/Hdiv4bHuyoPWIBk48n5ODYPZjrFCSbko1OEcwhB97ZoFs9VyzmsHKYov8EXx3+WI=";
}
