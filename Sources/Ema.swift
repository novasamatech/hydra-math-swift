import hydra_dx

public enum HydraEmaMath {
    public static func emaIteratedPrice<GenericIntoRustString: IntoRustString>(_ prev_n: GenericIntoRustString, _ prev_d: GenericIntoRustString, _ incoming_n: GenericIntoRustString, _ incoming_d: GenericIntoRustString, _ iterations: UInt32, _ smoothing: GenericIntoRustString) -> RustString {
        RustString(ptr: __swift_bridge__$ema_iterated_price({ let rustString = prev_n.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = prev_d.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = incoming_n.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), { let rustString = incoming_d.intoRustString(); rustString.isOwned = false; return rustString.ptr }(), iterations, { let rustString = smoothing.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
    }
}
