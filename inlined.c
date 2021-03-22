#include <rte_mbuf.h>
#include <rte_ethdev.h>
#include <rte_ether.h>

void rte_pktmbuf_free_(struct rte_mbuf *packet) {
    rte_pktmbuf_free(packet);
}

struct rte_mbuf* rte_pktmbuf_alloc_(struct rte_mempool *mp) {
    return rte_pktmbuf_alloc(mp);
}

uint16_t rte_eth_tx_burst_(uint16_t port_id, uint16_t queue_id, struct rte_mbuf **tx_pkts, uint16_t nb_pkts) {
    return rte_eth_tx_burst(port_id, queue_id, tx_pkts, nb_pkts);
}

uint16_t rte_eth_rx_burst_(uint16_t port_id, uint16_t queue_id, struct rte_mbuf **rx_pkts, const uint16_t nb_pkts) {
    return rte_eth_rx_burst(port_id, queue_id, rx_pkts, nb_pkts);
}

uint16_t rte_mbuf_refcnt_read_(const struct rte_mbuf* m) {
    return rte_mbuf_refcnt_read(m);
}

uint16_t rte_mbuf_refcnt_update_(struct rte_mbuf* m, int16_t value) {
    return rte_mbuf_refcnt_update(m, value);
}

char* rte_pktmbuf_adj_(struct rte_mbuf* m, uint16_t len) {
    return rte_pktmbuf_adj(m, len);
}

int rte_pktmbuf_trim_(struct rte_mbuf* m, uint16_t len) {
    return rte_pktmbuf_trim(m, len);
}

uint16_t rte_pktmbuf_headroom_(const struct rte_mbuf* m) {
    return rte_pktmbuf_headroom(m);
}

uint16_t rte_pktmbuf_tailroom_(const struct rte_mbuf* m) {
    return rte_pktmbuf_tailroom(m);
}
