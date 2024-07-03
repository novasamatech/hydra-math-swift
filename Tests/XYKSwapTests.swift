import XCTest
import HydraMathApi

final class XYKSwapTests: XCTestCase {
    func testCalculateInGivenOutCompletes() {
        let result = HydraXYKSwap.xykCalculateInGivenOut(
            "1000000000",
            "1000000000",
            "100000000"
        ).toString()
        
        XCTAssertTrue(result != "-1")
    }
    
    func testCalculateOutGivenInCompletes() {
        let result = HydraXYKSwap.xykCalculateOutGivenIn(
            "1000000000",
            "1000000000",
            "100000000"
        ).toString()
        
        XCTAssertTrue(result != "-1")
    }
    
    func testCalculatesFee() {
        let result = HydraXYKSwap.xykCalculatePoolTradeFee(
            "100000000",
            "1000000000",
            "1000000000"
        ).toString()
        
        XCTAssertTrue(result != "-1")
    }
}
