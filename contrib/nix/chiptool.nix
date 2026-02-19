{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, stdenv
}:

rustPlatform.buildRustPackage rec {
  pname = "chiptool";
  version = "0.1.0-dev";

  src = fetchFromGitHub {
    owner = "embassy-rs";
    repo = "chiptool";
    rev = "6a8c2aa32e84baf71a8482bd0dda671ca95e7207";
    hash = "sha256-dtPc89Vryk7SD2gbF8L5TFJslVeM3f/Hz/TQoMG3I/4=";
  };

  cargoLock = {
    lockFile = ./chiptool-Cargo.lock;
    outputHashes = {
      "svd-parser-0.14.9" = "sha256-z7vF93ihmCCM8Fm9DO4FgvmN22CXyFaJjCrV+aMpS7c=";
      "svd-rs-0.14.12" = "sha256-z7vF93ihmCCM8Fm9DO4FgvmN22CXyFaJjCrV+aMpS7c=";
    };
  };
  doCheck = false;

  nativeBuildInputs = [
    pkg-config
  ];

  # 如果构建失败，可能需要添加系统依赖到 buildInputs
  buildInputs = [ ];

  meta = with lib; {
    description = "Tool for generating Rust register maps from SVD files";
    homepage = "https://github.com/embassy-rs/chiptool";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "chiptool";
  };
}
