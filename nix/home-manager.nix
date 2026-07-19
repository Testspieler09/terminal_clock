{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.terminal-clock;

  tomlFormat = pkgs.formats.toml { };

  quoteType = lib.types.submodule {
    options = {
      text = lib.mkOption {
        type        = lib.types.str;
        description = "The quote text.";
      };
      author = lib.mkOption {
        type        = lib.types.nullOr lib.types.str;
        default     = null;
        description = "Optional author name.";
      };
      accent_color = lib.mkOption {
        type        = lib.types.nullOr lib.types.str;
        default     = null;
        description = "Optional color string (e.g. \"#ff0000\" or \"Red\").";
      };
      initial = lib.mkOption {
        type        = lib.types.bool;
        default     = false;
        description = "Set to true on exactly one quote to make it the first one shown on startup.";
      };
    };
  };

in
{
  options.programs.terminal-clock = {
    enable = lib.mkEnableOption "terminal-clock";

    package = lib.mkOption {
      type        = lib.types.package;
      default     = pkgs.terminal-clock;
      description = "The terminal-clock package to use.";
    };

    settings = {
      clock_face = lib.mkOption {
        type        = lib.types.nullOr lib.types.str;
        default     = null;
        description = "Name of the clock face to use (bundled or user-defined).";
      };
      color_theme = lib.mkOption {
        type        = lib.types.nullOr lib.types.str;
        default     = null;
        description = "Name of the color theme to use (bundled or user-defined).";
      };
      time_format = lib.mkOption {
        type        = lib.types.nullOr (lib.types.enum [ "HH:MM:SS" "HH:MM" ]);
        default     = null;
        description = "Clock time format.";
      };
      show_quote = lib.mkOption {
        type        = lib.types.nullOr lib.types.bool;
        default     = null;
        description = "Whether to show a quote below the clock.";
      };
      refresh_rate = lib.mkOption {
        type        = lib.types.nullOr lib.types.ints.positive;
        default     = null;
        description = "Refresh rate in milliseconds.";
      };
    };

    quotes = lib.mkOption {
      type        = lib.types.listOf quoteType;
      default     = [ ];
      description = "Extra quotes added on top of the bundled quote list.";
    };

    clockFaces = lib.mkOption {
      type        = lib.types.attrsOf tomlFormat.type;
      default     = { };
      description = "User-defined clock face definitions, written to clock_faces/<name>.toml.";
      example     = lib.literalExpression ''
        {
          my_face = {
            clock_type = "ColorClock";
            config = { ... };
          };
        }
      '';
    };

    colorThemes = lib.mkOption {
      type        = lib.types.attrsOf tomlFormat.type;
      default     = { };
      description = "User-defined color themes, written to themes/<name>.toml.";
      example     = lib.literalExpression ''
        {
          my_theme = {
            name       = "my_theme";
            foreground = "#cdd6f4";
            background = "#1e1e2e";
            selection  = "#585b70";
            accent     = "#89b4fa";
            borders    = "#313244";
          };
        }
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile = lib.mkMerge [
      (lib.mkIf (lib.any (v: v != null) (lib.attrValues {
        inherit (cfg.settings)
          clock_face color_theme time_format show_quote refresh_rate;
      })) {
        "terminal_clock/tc.toml".source = tomlFormat.generate "tc.toml" (
          lib.filterAttrs (_: v: v != null) {
            inherit (cfg.settings)
              clock_face color_theme time_format show_quote refresh_rate;
          }
        );
      })

      (lib.mkIf (cfg.quotes != [ ]) {
        "terminal_clock/quotes.toml".source =
          tomlFormat.generate "quotes.toml" { quote = cfg.quotes; };
      })

      (lib.mapAttrs' (name: face: {
        name  = "terminal_clock/clock_faces/${name}.toml";
        value.source = tomlFormat.generate "${name}.toml" face;
      }) cfg.clockFaces)

      (lib.mapAttrs' (name: theme: {
        name  = "terminal_clock/themes/${name}.toml";
        value.source = tomlFormat.generate "${name}.toml" theme;
      }) cfg.colorThemes)
    ];
  };
}
