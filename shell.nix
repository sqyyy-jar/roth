{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
    buildInputs = with pkgs; [
        cargo-cross
        podman
        qemu
    ];
}
