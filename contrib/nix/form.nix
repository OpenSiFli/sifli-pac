{ lib
, rustPlatform
, fetchCrate
}:

rustPlatform.buildRustPackage rec {
  pname = "form";
  version = "0.13.0"; # 可能需要更新到最新版本

  src = fetchCrate {
    inherit pname version;
    hash = "sha256-7+5HEyP7480UM5dydavoiclX3YTvW46V8r+Vpqt4xWk="; # 第一次构建时会提示真实的 hash
  };

  cargoHash = "sha256-ItNBQKye1GD01KFBubMLxksv8OCWIxya/LlZ9g6Jdg8="; # 第一次构建时会提示真实的 hash

  meta = with lib; {
    description = "A configurable code formatter for Rust and other languages";
    homepage = "https://crates.io/crates/form";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "form";
  };
}
