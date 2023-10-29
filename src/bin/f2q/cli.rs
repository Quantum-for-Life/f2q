use clap::{
    Args,
    Parser,
    Subcommand,
    ValueEnum,
};

/// Fermion to qubit mappings
#[derive(Debug, Parser)]
#[command(name = "f2q", version = "0.3.0")]
pub struct Cli {
    #[arg(long, short, default_value = "false")]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generates Hamiltonian
    #[command(arg_required_else_help = true)]
    #[command(short_flag = 'G')]
    Generate(Generate),
    #[command(short_flag = 'C')]
    Convert(Convert),
}

#[derive(Debug, Args)]
pub struct Generate {
    #[arg(short, long, required = true)]
    pub random:            bool,
    #[arg(short, long, default_value = "qubits")]
    pub encoding:          Encoding,
    #[arg(short, long, default_value = "json")]
    pub format:            Format,
    /// Pretty print the output if possible
    #[arg(short, long, default_value = "false")]
    pub pretty_print:      bool,
    pub num_terms:         u64,
    #[arg(long, default_value = "63")]
    pub max_orbital_index: u32,
    /// STDOUT, if not specified
    #[arg(long, short)]
    pub output_file:       Option<String>,
}

#[derive(Debug, Args)]
pub struct Convert {
    /// STDIN, if not specified
    #[arg(long, short)]
    pub input_file:      Option<String>,
    #[arg(long, default_value = "fermions")]
    pub input_encoding:  Encoding,
    #[arg(long, default_value = "json")]
    pub input_format:    Format,
    /// STDOUT, if not specified
    #[arg(long, short)]
    pub output_file:     Option<String>,
    #[arg(long, default_value = "qubits")]
    pub output_encoding: Encoding,
    #[arg(long, default_value = "json")]
    pub output_format:   Format,
    /// Pretty print the output if possible
    #[arg(short, long, default_value = "false")]
    pub pretty_print:    bool,
    #[arg(short, long)]
    pub mapping:         Mapping,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Encoding {
    Qubits,
    Fermions,
}

impl std::fmt::Display for Encoding {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mapping {
    JordanWigner,
}

impl std::fmt::Display for Mapping {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl std::fmt::Display for Format {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
