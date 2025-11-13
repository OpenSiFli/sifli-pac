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
    owner = "decaday";
    repo = "chiptool";
    rev = "dev"; # 使用 dev 分支
    hash = "sha256-HNbmox5f0LASTC0x3nK4IsALtgfq761VONuuVpszrTk=";
  };

  cargoLock = {
    lockFile = ./chiptool-Cargo.lock;
    outputHashes = {
      "svd-parser-0.14.5" = "sha256-0q5nNLsaorqW4n47y1V5YkmlnatPUINcHRXOSFkhmfo=";
      "svd-rs-0.14.7" = "sha256-0q5nNLsaorqW4n47y1V5YkmlnatPUINcHRXOSFkhmfo=";
    };
  };
  doCheck = false;

  nativeBuildInputs = [
    pkg-config
  ];

  # 如果构建失败，可能需要添加系统依赖到 buildInputs
  buildInputs = [ ];

  meta = with lib; {
    description = "Tool for generating Rust register maps from SVD files (forked by decaday)";
    homepage = "https://github.com/decaday/chiptool";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "chiptool";
  };
}
