{ kor, mkCargoNix }:
let
  klomziKreits = (
    mkCargoNix {
      cargoNix = import ./Cargo.nix;
      nightly = true;
    }
  ).workspaceMembers;

in
{ }
