import XCTest
import HydraMathApi

class StableswapTests: XCTestCase {
    func testCalculateOutGiveInShouldWorkWhenCorrectJsonFormatProvided() {
        let data = """
        [{
            "asset_id": 1,
            "amount": "1000000000000",
            "decimals": 12
        },
        {
            "asset_id": 0,
            "amount": "1000000000000",
            "decimals": 12
        }
        ]
        """
        
        let result = HydraStableswapMath.stableswapCalculateOutGivenIn(
            data,
            0,
            1,
            "1000000000",
            "1",
            "0",
            "[[\"1\", \"1\"], [\"1\", \"1\"]]"
        )
        
        XCTAssertEqual(result.toString(), "999500248")
    }
    
    func testCalculateSharesShouldWorkWhenCorrectJsonFormatProvided() {
        let data = """
        [{
            "asset_id": 0,
            "amount":"90000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount": "5000000000000000000000",
            "decimals": 12
        }
        ]
        """
        
        let assets = """
        [{"asset_id":1,"amount":"43000000000000000000"}]
        """
        
        let result = HydraStableswapMath.stableswapCalculateShares(
            data,
            assets,
            "1000",
            "64839594451719860",
            "0",
            "[[\"1\", \"1\"], [\"1\", \"1\"]]"
        )
        
        XCTAssertEqual(result.toString(), "371541351762585")
    }
}
