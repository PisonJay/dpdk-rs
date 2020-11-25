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
    fn rte_pmd_mlx5_get_dyn_flag_names();
}

#[inline(never)]
pub fn load_mlx5_driver() {
    if std::env::var("DONT_SET_THIS").is_ok() {
        unsafe { rte_pmd_mlx5_get_dyn_flag_names(); }
    }
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
    use std::ptr;
    #[test]
    fn it_works() {
        unsafe {
            crate::mlx5_rx_burst(ptr::null_mut(), ptr::null_mut(), 0);
        }
    }
}
