{ pkgs, ... }:
let
  lua = pkgs.fetchFromGitHub {
    owner = "lua";
    repo = "lua";
    rev = "9f791535cde1539338f6ba1b9154006595d97fe7";
    sha256 = "sha256-jcu37zun7ibKmK0uquzwYaoJh5miiWvDvAG/LOuexTI=";
  };
  sol2 = pkgs.fetchFromGitHub {
    owner = "ThePhD";
    repo = "sol2";
    rev = "9190880c593dfb018ccf5cc9729ab87739709862";
    sha256 = "sha256-bW6HD9WLOWizli6LnrkFZKxiT8IdN0QESlok+xCFz1w=";
  };
in
{
  setup_script = ''
    mkdir -p deps
    cp -r ${lua} deps/lua
    cp -r ${sol2} deps/sol2
  '';
}
