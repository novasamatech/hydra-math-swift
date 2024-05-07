// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "HydraMath",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "HydraMath",
            targets: ["HydraMath"]),
    ],
    dependencies: [],
    targets: [
        .binaryTarget(
            name: "hydra_dx",
            path: "./HydraMath/xcframework/hydra_dx.xcframework"),
        .target(
            name: "HydraMath",
            dependencies: ["hydra_dx"],
            path: "HydraMath/Classes",
            resources: []
        )
    ]
)