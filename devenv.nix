{pkgs, ...}: {
  # https://devenv.sh/packages/
  packages = [pkgs.git];

  # https://devenv.sh/languages/
  languages = {
    rust.enable = true;
  };

  # https://devenv.sh/integrations/difftastic/
  difftastic.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  pre-commit = {
    hooks = {
      alejandra.enable = true;
      rustfmt.enable = true;
      clippy.enable = true;
      typos.enable = true;
    };

    settings = {
      clippy = {
        allFeatures = true;
        denyWarnings = true;
      };
    };
  };

  # See full reference at https://devenv.sh/reference/options/
}
