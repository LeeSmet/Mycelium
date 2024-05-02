//! A dedicated subnet module.
//!
//! The standard library only exposes [`IpAddr`], and types related to
//! specific IPv4 and IPv6 addresses. It does not however, expose dedicated types to represent
//! appropriate subnets.
//!
//! This code is not meant to fully support subnets, but rather only the subset as needed by the
//! main application code. As such, this implementation is optimized for the specific use case, and
//! might not be optimal for other uses.

use core::fmt;
use std::net::IpAddr;

use ipnet::IpNet;

/// Representation of a subnet. A subnet can be either IPv4 or IPv6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subnet {
    inner: IpNet,
}

/// An error returned when creating a new [`Subnet`] with an invalid prefix length.
///
/// For IPv4, the max prefix length is 32, and for IPv6 it is 128;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixLenError;

impl Subnet {
    /// Create a new `Subnet` from the given [`IpAddr`] and prefix length.
    pub fn new(addr: IpAddr, prefix_len: u8) -> Result<Subnet, PrefixLenError> {
        Ok(Self {
            inner: IpNet::new(addr, prefix_len).map_err(|_| PrefixLenError)?,
        })
    }

    /// Returns the size of the prefix in bits.
    pub fn prefix_len(&self) -> u8 {
        self.inner.prefix_len()
    }

    /// Retuns the address in this subnet.
    ///
    /// The returned address is a full IP address, used to construct this `Subnet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mycelium::subnet::Subnet;
    /// use std::net::Ipv6Addr;
    ///
    /// let address = Ipv6Addr::new(12,34,56,78,90,0xab,0xcd,0xef).into();
    /// let subnet = Subnet::new(address, 64).unwrap();
    ///
    /// assert_eq!(subnet.address(), address);
    /// ```
    pub fn address(&self) -> IpAddr {
        self.inner.addr()
    }

    /// Checks if this `Subnet` contains the provided `Subnet`, i.e. all addresses of the provided
    /// `Subnet` are also part of this `Subnet`
    ///
    /// # Examples
    ///
    /// ```
    /// use mycelium::subnet::Subnet;
    /// use std::net::Ipv4Addr;
    ///
    /// let global = Subnet::new(Ipv4Addr::new(0,0,0,0).into(), 0).expect("Defined a valid subnet");
    /// let local = Subnet::new(Ipv4Addr::new(10,0,0,0).into(), 8).expect("Defined a valid subnet");
    ///
    /// assert!(global.contains_subnet(&local));
    /// assert!(!local.contains_subnet(&global));
    /// ```
    pub fn contains_subnet(&self, other: &Self) -> bool {
        self.inner.contains(&other.inner)
    }

    /// Checks if this `Subnet` contains the provided [`IpAddr`].
    ///
    /// # Examples
    ///
    /// ```
    /// use mycelium::subnet::Subnet;
    /// use std::net::{Ipv4Addr,Ipv6Addr};
    ///
    /// let ip_1 = Ipv6Addr::new(12,34,56,78,90,0xab,0xcd,0xef).into();
    /// let ip_2 = Ipv6Addr::new(90,0xab,0xcd,0xef,12,34,56,78).into();
    /// let ip_3 = Ipv4Addr::new(10,1,2,3).into();
    /// let subnet = Subnet::new(Ipv6Addr::new(12,34,5,6,7,8,9,0).into(), 32).unwrap();
    ///
    /// assert!(subnet.contains_ip(ip_1));
    /// assert!(!subnet.contains_ip(ip_2));
    /// assert!(!subnet.contains_ip(ip_3));
    /// ```
    pub fn contains_ip(&self, ip: IpAddr) -> bool {
        self.inner.contains(&ip)
    }

    /// Returns the network part of the `Subnet`. All non prefix bits are set to 0.
    pub fn network(&self) -> IpAddr {
        self.inner.network()
    }

    /// Returns the braodcast address for the subnet.
    pub fn broadcast_addr(&self) -> IpAddr {
        self.inner.broadcast()
    }

    /// Returns the netmask of the subnet as an [`IpAddr`].
    pub fn mask(&self) -> IpAddr {
        self.inner.netmask()
    }
}

impl fmt::Display for Subnet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Display for PrefixLenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid prefix length for this address")
    }
}

impl std::error::Error for PrefixLenError {}
