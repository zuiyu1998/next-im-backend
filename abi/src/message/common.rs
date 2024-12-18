use std::net::{IpAddr, SocketAddr};

use network_interface::NetworkInterfaceConfig;
use url::Url;

use super::{platform, IpVersion};
use crate::{Error, ErrorKind, Result};

pub(crate) fn get_interface_name_by_ip(local_ip: &IpAddr) -> Option<String> {
    if local_ip.is_unspecified() || local_ip.is_multicast() {
        return None;
    }
    let ifaces = network_interface::NetworkInterface::show().ok()?;
    for iface in ifaces {
        for addr in iface.addr {
            if addr.ip() == *local_ip {
                return Some(iface.name);
            }
        }
    }

    tracing::error!(?local_ip, "can not find interface name by ip");
    None
}

pub(crate) fn setup_sokcet2_ext(
    socket2_socket: &socket2::Socket,
    bind_addr: &SocketAddr,
    #[allow(unused_variables)] bind_dev: Option<String>,
) -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    {
        let is_udp = matches!(socket2_socket.r#type()?, socket2::Type::DGRAM);
        platform::windows::setup_socket_for_win(socket2_socket, bind_addr, bind_dev, is_udp)?;
    }

    if bind_addr.is_ipv6() {
        socket2_socket.set_only_v6(true)?;
    }

    socket2_socket.set_nonblocking(true)?;
    socket2_socket.set_reuse_address(true)?;
    socket2_socket.bind(&socket2::SockAddr::from(*bind_addr))?;

    if bind_addr.ip().is_unspecified() {
        return Ok(());
    }

    Ok(())
}

pub(crate) fn setup_sokcet2(
    socket2_socket: &socket2::Socket,
    bind_addr: &SocketAddr,
) -> Result<(), Error> {
    setup_sokcet2_ext(
        socket2_socket,
        bind_addr,
        get_interface_name_by_ip(&bind_addr.ip()),
    )
}

pub(crate) trait FromUrl {
    fn from_url(url: Url, ip_version: IpVersion) -> Result<Self, Error>
    where
        Self: Sized;
}

impl FromUrl for SocketAddr {
    fn from_url(url: url::Url, ip_version: IpVersion) -> Result<Self, Error> {
        let addrs = url.socket_addrs(|| None)?;
        tracing::debug!(?addrs, ?ip_version, ?url, "convert url to socket addrs");
        let addrs = addrs
            .into_iter()
            .filter(|addr| match ip_version {
                IpVersion::V4 => addr.is_ipv4(),
                IpVersion::V6 => addr.is_ipv6(),
                IpVersion::Both => true,
            })
            .collect::<Vec<_>>();

        use rand::seq::SliceRandom;
        // randomly select one address
        addrs
            .choose(&mut rand::thread_rng())
            .copied()
            .ok_or(ErrorKind::NoDnsRecordFound(ip_version).into())
    }
}

pub(crate) fn check_scheme_and_get_socket_addr<T>(url: &url::Url, scheme: &str) -> Result<T>
where
    T: FromUrl,
{
    if url.scheme() != scheme {
        return Err(ErrorKind::InvalidProtocol(url.scheme().to_string()).into());
    }

    Ok(T::from_url(url.clone(), IpVersion::Both)?)
}
