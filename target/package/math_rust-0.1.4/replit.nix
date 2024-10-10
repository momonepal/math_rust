{ pkgs }: {
	deps = [
		pkgs.sudo
  pkgs.vlang
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}