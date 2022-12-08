{ pkgs ? import <nixpkgs> { } }:
let
  builder = pkgs.writeScriptBin "build" ''
    #!/usr/bin/env bash
    mode=$1

    case ''${mode,,} in
      release|r)
        cargo build --release && \
          sudo setcap cap_net_raw,cap_net_admin=eip "target/release/pkit"
        ;;
      debug|d)
        cargo build && \
          sudo setcap cap_net_raw,cap_net_admin=eip "target/debug/pkit"
        ;;
      *) echo unknow parameter "$(echo mode|sed "s/.*/\U\0/")" && exit 1 ;;
    esac
  '';

  psniff = pkgs.writeScriptBin "run" ''
    target/$1/pkit "''${@:2}"
  '';
in
pkgs.mkShell {
  name = "Psniff developement environment";
  description = ''
  This shell comes with two builtin scripts, `build` and `run`,

  # build
  The `build` script provides an easier way to build the toolkit and set appropriate permissions for running the application
   - It takes the profile ((r)elease|(d)ebug) and then builds the application
   - It automatically sets the required permissions for running the application

  # run
  The `run` script allow running the application without `cargo run -- $@`
   - It takes one positional argument to accept the profile (release|debug)
   - All arguments after the profile are directly passed to the application
  '';


  buildInputs = with pkgs; [
    builder
    libpcap
    pcapc
    psniff
  ];

  shellHook = ''
    echo $PWD
    echo Loaded psniff devel env via nix
  '';
}
