{ ... }@inputs: {
  pkg = inputs.pkgs.stdenv.mkDerivation rec {
    pname = "lua-pkg";
    version = "5.4.8";
    src = inputs.pkgs.fetchurl {
      url = "https://www.lua.org/ftp/lua-${version}.tar.gz";
      sha256 = "4f18ddae154e793e46eeab727c59ef1c0c0c2b744e7b94219710d76f530629ae";
    };
    nativeBuildInputs = [ inputs.pkgs.makeWrapper inputs.pkgs.pkg-config inputs.pkgs.readline ];
    buildInputs = [ ];

    configurePhase = ''make'';
    installPhase = ''
      		mkdir -p $out/bin $out/include $out/lib
      		cp src/lua src/luac $out/bin/
              cp src/lua.h src/lualib.h src/lauxlib.h src/luaconf.h src/lua.hpp $out/include/
              cp src/liblua.a $out/lib/
      	'';
  };
}
