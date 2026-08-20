import XCTest
import HydraMathApi

final class EmaTests: XCTestCase {
    private let tenMinutesSmoothing = "3369132345751865974884897103284833777"

    func testIteratedPriceUsesTheSmoothingItIsGiven() {
        let fullSmoothing = "170141183460469231731687303715884105728"

        let result = HydraEmaMath.emaIteratedPrice("1", "2", "3", "4", 5, fullSmoothing).toString()

        XCTAssertEqual(result, "12,16")
    }

    func testIteratedPriceKeepsPrevWhenNotOutdated() {
        let result = HydraEmaMath.emaIteratedPrice(
            "1",
            "2",
            "3",
            "4",
            0,
            tenMinutesSmoothing
        ).toString()

        XCTAssertEqual(result, "1,2")
    }

    func testIteratedPriceFailsOnInvalidInput() {
        let result = HydraEmaMath.emaIteratedPrice(
            "invalid",
            "2",
            "3",
            "4",
            10,
            tenMinutesSmoothing
        ).toString()

        XCTAssertEqual(result, "-1")
    }
}
