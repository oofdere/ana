// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterAna",
    products: [
        .library(name: "TreeSitterAna", targets: ["TreeSitterAna"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterAna",
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
            name: "TreeSitterAnaTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterAna",
            ],
            path: "bindings/swift/TreeSitterAnaTests"
        )
    ],
    cLanguageStandard: .c11
)
