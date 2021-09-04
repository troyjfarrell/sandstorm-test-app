@0x8072ba7986989eca;

using Spk = import "/sandstorm/package.capnp";

const pkgdef :Spk.PackageDefinition = (
  id = "cfwh4nxv4avz4p2254f3uwppmg54gfd0vsn2hudgws7de5p4710h",

  manifest = (
    appTitle = (defaultText = "Sandstorm Test App (Rust)"),

    appVersion = 0,
    appMarketingVersion = (defaultText = "0.0.0"),

    actions = [
      ( title = (defaultText = "New Test App Instance"),
        nounPhrase = (defaultText = "instance"),
        command = (argv = ["/opt/app/test-app"])
      )
    ],

    continueCommand = (argv = ["/opt/app/test-app"])
  ),

  sourceMap = (
    searchPath = [
      ( packagePath = "test-app", sourcePath = "../target/debug/test-app" ),
    ]
  ),

  alwaysInclude = [ "test-app", "sandstorm-manifest" ]
);
