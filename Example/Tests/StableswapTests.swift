import XCTest
import HydraMath

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
        
        let result = HydraStableswapMath.calculateOutGivenIn(
            data,
            0,
            1,
            "1000000000",
            "1",
            "0"
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
        
        let result = HydraStableswapMath.calculateShares(
            data,
            assets,
            "1000",
            "64839594451719860",
            "0"
        )
        
        XCTAssertEqual(result.toString(), "371541351762585")
    }
    
    func testCalculateShareForAmountShouldReturnCorrectShares() {
        let data = """
        [{
            "asset_id": 0,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 2,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 3,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 4,
            "amount":"10000000000000000",
            "decimals": 12
        }
        ]
        """

        let result = HydraStableswapMath.calculateSharesForAmount(
            data,
            0,
            "100000000000000",
            "100",
            "20000000000000000000000",
            "0"
        )
        
        XCTAssertEqual(result.toString(), "40001593768209443008")
    }
    
    func testCalculateAddOneAsset() {
        let data = """
        [{
            "asset_id": 0,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 1,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 2,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 3,
            "amount":"10000000000000000",
            "decimals": 12
        },
        {
            "asset_id": 4,
            "amount":"10000000000000000",
            "decimals": 12
        }
        ]
        """
        
        let result = HydraStableswapMath.calculateAddOneAsset(
            data,
            "399850144492663029649",
            2,
            "100",
            "20000000000000000000000",
            "0"
        )
        
        XCTAssertEqual(result.toString(), "1000000000000000")
    }
}
