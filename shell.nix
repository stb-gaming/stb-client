with (import <nixpkgs> {});
mkShell {
  packages = [pkg-config];
  PKG_CONFIG_PATH = with builtins; concatStringsSep ":" (map (package:"${package.dev}/lib/pkgconfig") [
    openssl
  ]);
}