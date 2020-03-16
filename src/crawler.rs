use std::net::{Ipv4Addr,SocketAddr};


use super::{errors::Result,nmap::{Cidr,Run}, orm::Connection};

pub fn run(db: &Connection) -> Result<()> {
    for it in interfaces()? {        
        for ip in ip4(&it)? {            
            for nm in netmask4(&it)?{
                info!("find interface {} {}/{}", it, ip, nm);                
                for ih in Run::scan(&ip.to_string(), nm.to_string().parse::<Cidr>()?.0)?.hosts{
                    info!("find host {:?}", ih);
                }
            }
        }
    }
    Ok(())
}

fn ip4(name: &String) -> Result<Vec<Ipv4Addr>> {
    let items = nix::ifaddrs::getifaddrs()?
        .filter(|x| x.interface_name == *name)
        .map(|x| {
            if let Some(addr) = x.address {
                if let nix::sys::socket::SockAddr::Inet(addr) = addr {
                    if let SocketAddr::V4(addr) = addr.to_std() {
                        return Some(addr.ip().clone());
                    }
                }
            }
            None
        })
        .filter(|x| *x != None)
        .map(|x|x.unwrap())
        .collect::<Vec<_>>();
    Ok(items)
}

fn netmask4(name: &String) -> Result<Vec<Ipv4Addr>> {
    let items = nix::ifaddrs::getifaddrs()?
        .filter(|x| x.interface_name == *name)
        .map(|x| {
            if let Some(addr) = x.netmask {
                if let nix::sys::socket::SockAddr::Inet(addr) = addr {
                    if let SocketAddr::V4(addr) = addr.to_std() {
                        return Some(addr.ip().clone());
                    }
                }
            }
            None
        })
        .filter(|x| *x != None)
        .map(|x|x.unwrap())
        .collect::<Vec<_>>();
    Ok(items)
}

fn interfaces() -> Result<Vec<String>> {
    let mut items = nix::ifaddrs::getifaddrs()?
        .filter(|x| {
            x.flags.contains(nix::net::if_::InterfaceFlags::IFF_UP)
                && x.flags.contains(nix::net::if_::InterfaceFlags::IFF_RUNNING)
                && x.flags
                    .contains(nix::net::if_::InterfaceFlags::IFF_BROADCAST)
                && x.flags
                    .contains(nix::net::if_::InterfaceFlags::IFF_MULTICAST)
        })
        .map(|x| x.interface_name)
        .collect::<Vec<_>>();

    items.sort();
    items.dedup();
    Ok(items)
}
