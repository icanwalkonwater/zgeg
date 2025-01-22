// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterZgeg",
    products: [
        .library(name: "TreeSitterZgeg", targets: ["TreeSitterZgeg"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterZgeg",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
                // NOTE: if your language has an external scanner, add it here.
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterZgegTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterZgeg",
            ],
            path: "bindings/swift/TreeSitterZgegTests"
        )
    ],
    cLanguageStandard: .c11
)
