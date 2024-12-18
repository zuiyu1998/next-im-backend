use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
};

use futures::{stream::FuturesUnordered, StreamExt};
use network_interface::NetworkInterfaceConfig;
use url::Url;

use super::{platform, IpVersion};
use crate::{Error, ErrorKind, Result};

pub fn build_url_from_socket_addr(addr: &String, scheme: &str) -> url::Url {
    if let Ok(sock_addr) = addr.parse::<SocketAddr>() {
        let mut ret_url = url::Url::parse(format!("{}://0.0.0.0", scheme).as_str()).unwrap();
        ret_url.set_ip_host(sock_addr.ip()).unwrap();
        ret_url.set_port(Some(sock_addr.port())).unwrap();
        ret_url
    } else {
        url::Url::parse(format!("{}://{}", scheme, addr).as_str()).unwrap()
    }
}

pub(crate) fn check_scheme_and_get_socket_addr_ext<T>(
    url: &url::Url,
    scheme: &str,
    ip_version: IpVersion,
) -> Result<T, Error>
where
    T: FromUrl,
{
    if url.scheme() != scheme {
        return Err(ErrorKind::InvalidProtocol(url.scheme().to_string()).into());
    }

    Ok(T::from_url(url.clone(), ip_version)?)
}

pub(crate) async fn wait_for_connect_futures<Fut, Ret, E>(
    mut futures: FuturesUnordered<Fut>,
) -> Result<Ret, Error>
where
    Fut: Future<Output = Result<Ret, E>> + Send + Sync,
    E: std::error::Error + Into<Error> + Send + Sync + 'static,
{
    // return last error
    let mut last_err = None;

    while let Some(ret) = futures.next().await {
        if let Err(e) = ret {
            last_err = Some(e.into());
        } else {
            return ret.map_err(|e| e.into());
        }
    }

    Err(last_err.unwrap_or(ErrorKind::Shutdown.into()))
}

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
