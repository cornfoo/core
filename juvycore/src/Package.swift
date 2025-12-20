// swift-tools-version: 6.0

import PackageDescription

let package = Package(
  name: "JuvyCore",
  platforms: [
    .iOS(.v17)
  ],
  products: [
    .library(
      name: "JuvyCore",
      targets: ["JuvyCore", "JuvyCoreFFI"]
    )
  ],
  dependencies: [],
  targets: [
    .target(
      name: "JuvyCore",
      dependencies: ["JuvyCoreFFI"],
      swiftSettings: [
        .swiftLanguageMode(.v6)
      ]
    ),
    .binaryTarget(name: "JuvyCoreFFI", path: "Sources/JuvyCoreFFI.xcframework"),
  ]
)
