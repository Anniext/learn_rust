use clap::Parser;
use rcli::{Opts, SubCommand, process_csv, process_xlsx};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output: String = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::Xlsx(opts) => {
            let output_dir = opts.output_dir.as_deref().unwrap_or(".");
            process_xlsx(
                &opts.input,
                output_dir,
                opts.format,
                !opts.keep_empty_rows,
                !opts.keep_whitespace,
            )?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
