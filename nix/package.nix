{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname   = "terminal-clock";
  version = (lib.importTOML ./../Cargo.toml).workspace.package.version;

  src = ./..;

  cargoLock.lockFile = ./../Cargo.lock;

  cargoBuildFlags = [ "-p" "tc" ];
  cargoTestFlags  = [ "-p" "tc-user_config_loader" "-p" "tc-models" ];

  meta = {
    description = "A simple but fancy looking customizable terminal clock";
    license     = lib.licenses.mit;
    mainProgram = "tc";
  };
}
