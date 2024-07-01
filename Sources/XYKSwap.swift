import hydra_dx

public enum HydraXYKSwap {
    public static func xykCalculateOutGivenIn<GenericIntoRustString: IntoRustString>(_ balance_in: GenericIntoRustString, _ balance_out: GenericIntoRustString, _ amount_in: GenericIntoRustString) -> RustString {
        RustString(ptr: __swift_bridge__$xyk_calculate_out_given_in({ let rustString = balance_in.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = balance_out.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = amount_in.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
    public static func xykCalculateInGivenOut<GenericIntoRustString: IntoRustString>(_ balance_in: GenericIntoRustString, _ balance_out: GenericIntoRustString, _ amount_out: GenericIntoRustString) -> RustString {
        RustString(ptr: __swift_bridge__$xyk_calculate_in_given_out({ let rustString = balance_in.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = balance_out.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = amount_out.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
    public static func xykCalculatePoolTradeFee<GenericIntoRustString: IntoRustString>(_ amount: GenericIntoRustString, _ fee_nominator: GenericIntoRustString, _ fee_denominator: GenericIntoRustString) -> RustString {
        RustString(ptr: __swift_bridge__$xyk_calculate_pool_trade_fee({ let rustString = amount.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = fee_nominator.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = fee_denominator.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
