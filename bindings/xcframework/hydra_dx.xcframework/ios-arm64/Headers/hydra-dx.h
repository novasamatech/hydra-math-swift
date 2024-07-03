// File automatically generated by swift-bridge.
#include <stdint.h>
void* __swift_bridge__$stableswap_calculate_out_given_in(void* reserves, uint32_t asset_in, uint32_t asset_out, void* amount_in, void* amplification, void* fee);
void* __swift_bridge__$stableswap_calculate_in_given_out(void* reserves, uint32_t asset_in, uint32_t asset_out, void* amount_out, void* amplification, void* fee);
void* __swift_bridge__$stableswap_calculate_amplification(void* initial_amplification, void* final_amplification, void* initial_block, void* final_block, void* current_block);
void* __swift_bridge__$stableswap_calculate_shares(void* reserves, void* assets, void* amplification, void* share_issuance, void* fee);
void* __swift_bridge__$stableswap_calculate_shares_for_amount(void* reserves, uint32_t asset_in, void* amount, void* amplification, void* share_issuance, void* fee);
void* __swift_bridge__$stableswap_calculate_add_one_asset(void* reserves, void* shares, uint32_t asset_in, void* amplification, void* share_issuance, void* fee);
void* __swift_bridge__$stableswap_calculate_liquidity_out_one_asset(void* reserves, void* shares, uint32_t asset_out, void* amplification, void* share_issuance, void* withdraw_fee);
void* __swift_bridge__$xyk_calculate_out_given_in(void* balance_in, void* balance_out, void* amount_in);
void* __swift_bridge__$xyk_calculate_in_given_out(void* balance_in, void* balance_out, void* amount_out);
void* __swift_bridge__$xyk_calculate_pool_trade_fee(void* amount, void* fee_nominator, void* fee_denominator);

