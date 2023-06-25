use crate::common::*;

/// Compile and flash Rust firmware onto Texas Instruments microcontrollers.
#[derive(Debug, Default, Parser)]
#[command(version, override_usage = "cargo-uniflash [<cargo_args…>] [-- <args…>]")]
pub struct RawArgs {
    /// Print all built-in board configurations.
    #[arg(long)]
    pub list_boards: bool,

    /// The chosen built-in board to flash.
    #[arg(long, conflicts_with = "custom_config")]
    pub board: Option<Board>,

    /// Path to a custom device configuration.
    #[arg(long)]
    pub custom_config: Option<PathBuf>,

    /// Path to an ELF image to flash (skips `cargo build` step).
    #[arg(long)]
    pub elf: Option<PathBuf>,

    /// Verify program after flashing.
    #[arg(long)]
    pub verify: bool,
}

impl RawArgs {
    pub fn parse(args: impl Iterator<Item = String>) -> anyhow::Result<(Vec<String>, Self)> {
        let mut args = args.peekable();
        let prog = args.next().unwrap();

        // flush the next arg if we are running as a cargo subcommand
        if matches!(args.peek().map(|a| &**a), Some("uniflash")) {
            _ = args.next();
        }

        // chomp all args before the first `--` to pass to cargo...
        let cargo_args: Vec<_> = (&mut args).take_while(|arg| arg != "--").collect();

        // ...unless they contain `--help`, in which case print the `cargo-uniflash` help
        let mut args: Vec<_> = iter::once(prog).chain(&mut args).collect();
        if let Some(help) = cargo_args.iter().find(|arg| matches!(arg.as_str(), "-h" | "--help")) {
            args.push(help.to_owned());
        }

        // pull default values from the environment, but allow overrides
        // from the command line
        let mut args = RawArgs::parse_from(args);
        args.merge_from_env()?;

        if args.list_boards {
            Board::print_all();
        }
        if args.num_targets() == 0 {
            anyhow::bail!("At least one flash target must be specified");
        }

        debug_assert!(args.num_targets() == 1);
        Ok((cargo_args, args))
    }
}

impl RawArgs {
    fn from_env() -> anyhow::Result<Self> {
        let mut args = Self::default();

        if let Ok(board) = env::var("CARGO_UNIFLASH_BOARD") {
            args.board = Some(board.parse()?);
        }
        if let Ok(custom_config) = env::var("CARGO_UNIFLASH_CUSTOM_CONFIG") {
            args.custom_config = Some(PathBuf::from(custom_config));
        }
        if args.num_targets() > 1 {
            anyhow::bail!("Can't set both `board` and `custom config` from env simultaneously");
        }

        Ok(args)
    }

    fn merge_from_env(&mut self) -> anyhow::Result<()> {
        let env_args = Self::from_env()?;

        if self.num_targets() == 0 {
            debug_assert!(env_args.num_targets() <= 1);
            self.board = env_args.board;
            self.custom_config = env_args.custom_config;
        }

        Ok(())
    }

    fn num_targets(&self) -> usize {
        usize::from(self.board.is_some()) + usize::from(self.custom_config.is_some())
    }
}
