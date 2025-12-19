// swift-tools-version: 6.0

import PackageDescription

let package = Package(
  name: "Juvy",
  platforms: [
    .iOS(.v17)
  ],
  products: [
    .library(
      name: "Juvy",
      targets: ["Juvy", "JuvyFFI"]
    )
  ],
  dependencies: [],
  targets: [
    .target(
      name: "Juvy",
      dependencies: ["JuvyFFI"],
      swiftSettings: [
        .swiftLanguageMode(.v6)
      ]
    ),
    .binaryTarget(name: "JuvyFFI", path: "Sources/JuvyFFI.xcframework"),
  ]
)
