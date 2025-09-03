// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "HydraMathApi",
    products: [
        .library(
            name: "HydraMathApi",
            targets: ["HydraMathApi"]
        ),
    ],
    targets: [
        .binaryTarget(
            name: "hydra_dx",
            path: "./bindings/xcframework/hydra_dx.xcframework"
        ),
        .target(
            name: "HydraMathApi",
            dependencies: ["hydra_dx"],
            path: "Sources"
        ),
        .testTarget(
            name: "Tests",
            dependencies: [
                "HydraMathApi"
            ],
            path: "Tests"
        )
    ]
)
