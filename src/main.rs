use clap::Parser;
use fizzbuzz::cli::Cli;
use fizzbuzz::generator::Generator;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

fn main() {
    let cli: Cli = Cli::parse();

    let gen = Generator::new(
        Some(cli.min),
        cli.max,
        cli.defines
            .unwrap_or_else(|| vec![("Fizz".to_string(), 3), ("Buzz".to_string(), 5)]),
    );

    if cli.print_instantly {
        for val in gen {
            println!("{val}");
        }
    } else {
        // 128 bit computers don't exist yet, so this is safe for now
        let bar = ProgressBar::new(cli.max.try_into().expect("Usize too big")).with_style(
            ProgressStyle::default_bar()
                .template(
                    "[{elapsed_precise} | ETA {eta}] {bar:50.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .expect("failed to parse template")
                .progress_chars("##-"),
        );

        let value = gen
            .progress_with(bar.clone())
            .inspect(|val| {
                bar.set_message(val.clone());
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{value}");
    }
}
