import XCTest
import HydraMathApi

final class OmnipoolTests: XCTestCase {
    func testCalculateOutGivenInShouldWork() {
        let result = HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString()

        XCTAssertTrue(result != "-1")
    }

    func testCalculateOutGivenInWithFeeShouldReduceOutput() {
        let noFee = Int(HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString())!

        let withFee = Int(HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0.01",
            "0",
            "0"
        ).toString())!

        XCTAssertTrue(withFee < noFee)
    }

    func testCalculateOutGivenInWithSlipFeeShouldReduceOutput() {
        let noSlip = Int(HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString())!

        let withSlip = Int(HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0.05"
        ).toString())!

        XCTAssertTrue(withSlip < noSlip)
    }

    func testCalculateInGivenOutShouldWork() {
        let result = HydraOmnipoolMath.omnipoolCalculateInGivenOut(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString()

        XCTAssertTrue(result != "-1")
    }

    func testCalculateInGivenOutWithSlipFeeShouldIncreaseCost() {
        let noSlip = Int(HydraOmnipoolMath.omnipoolCalculateInGivenOut(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString())!

        let withSlip = Int(HydraOmnipoolMath.omnipoolCalculateInGivenOut(
            "1000000000000",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0.05"
        ).toString())!

        XCTAssertTrue(withSlip > noSlip)
    }

    func testCalculateOutGivenInReturnsErrorOnInvalidInput() {
        let result = HydraOmnipoolMath.omnipoolCalculateOutGivenIn(
            "invalid",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString()

        XCTAssertEqual(result, "-1")
    }

    func testCalculateInGivenOutReturnsErrorOnInvalidInput() {
        let result = HydraOmnipoolMath.omnipoolCalculateInGivenOut(
            "invalid",
            "500000000000",
            "1000000000000",
            "2000000000000",
            "800000000000",
            "2000000000000",
            "100000000000",
            "0",
            "0",
            "0"
        ).toString()

        XCTAssertEqual(result, "-1")
    }
}
