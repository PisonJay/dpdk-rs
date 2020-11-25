use std::ffi::c_void;

mod bindings;

pub use bindings::*;

#[link(name = "inlined")]
extern "C" {
    fn rte_pktmbuf_free_(packet: *mut rte_mbuf);
    fn rte_pktmbuf_alloc_(mp: *mut rte_mempool) -> *mut rte_mbuf;
    fn rte_eth_tx_burst_(port_id: u16, queue_id: u16, tx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16;
    fn rte_eth_rx_burst_(port_id: u16, queue_id: u16, rx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16;
}

#[link(name = "rte_net_mlx5")]
extern "C" {
    pub fn mlx5_rx_burst(dpdkp_rxq: *mut c_void, pkts: *mut *mut rte_mbuf, pkts_n: u16);
}

#[inline]
pub unsafe fn rte_pktmbuf_free(packet: *mut rte_mbuf) {
    rte_pktmbuf_free_(packet)
}

#[inline]
pub unsafe fn rte_pktmbuf_alloc(mp: *mut rte_mempool) -> *mut rte_mbuf {
    rte_pktmbuf_alloc_(mp)
}

#[inline]
pub unsafe fn rte_eth_tx_burst(port_id: u16, queue_id: u16, tx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16 {
    rte_eth_tx_burst_(port_id, queue_id, tx_pkts, nb_pkts)
}

#[inline]
pub unsafe fn rte_eth_rx_burst(port_id: u16, queue_id: u16, rx_pkts: *mut *mut rte_mbuf, nb_pkts: u16) -> u16 {
    rte_eth_rx_burst_(port_id, queue_id, rx_pkts, nb_pkts)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        unsafe {
            crate::rte_pktmbuf_free(std::ptr::null_mut()); 
        }
    }
}
