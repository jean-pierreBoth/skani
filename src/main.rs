use clap::{AppSettings, Arg, Command, SubCommand};
use skani::chain;
use skani::file_io;
use skani::types::*;
use std::time::Instant;

fn main() {
    let dist = "dist";
    let classify = "classify";
    let matches = Command::new("skani")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1")
        .about("skani")
        .subcommand(
            SubCommand::with_name(classify)
                .arg(
                    Arg::new("query")
                        .short('q')
                        .help("query fasta.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("reference")
                        .short('r')
                        .help("reference fasta.")
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(
                    Arg::new("k")
                        .short('k')
                        .help("k-mer size.")
                        .default_value("20")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("c")
                        .short('c')
                        .help("compression factor.")
                        .default_value("100")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("d")
                        .short('d')
                        .help("dowsample factor for open syncmers.")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("dist")
                .arg(
                    Arg::new("reference")
                        .index(1)
                        .help("reference fasta.")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::new("k")
                        .short('k')
                        .help("k-mer size.")
                        .default_value("20")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("c")
                        .short('c')
                        .help("compression factor.")
                        .default_value("100")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("d")
                        .short('d')
                        .help("dowsample factor for open syncmers. ")
                        .takes_value(true),
                ),
        )
        .get_matches();

    let mode;
    let matches_subc;
    match matches.subcommand_name() {
        Some("classify") => {
            mode = classify;
            matches_subc = matches.subcommand_matches(classify).unwrap()
        } // clone was used
        Some("dist") => {
            mode = dist;
            matches_subc = matches.subcommand_matches(dist).unwrap()
        } // push was used
        _ => {
            panic!()
        } // Either no subcommand or one not tested for...
    }

    let ref_files: Vec<&str>;
    if let Some(values) = matches_subc.values_of("reference") {
        ref_files = values.collect();
    } else {
        ref_files = vec![];
    }
    let k = matches_subc
        .value_of("k")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let c;
    let use_syncs;
    if !matches_subc.value_of("d").is_none() {
        use_syncs = true;
        c = matches_subc.value_of("d").unwrap().parse().unwrap();
    } else {
        use_syncs = false;
        c = matches_subc
            .value_of("c")
            .unwrap()
            .parse::<usize>()
            .unwrap();
    }
    dbg!(use_syncs, c);

    let now = Instant::now();
    let mut ref_sketches = vec![];
    for ref_file in ref_files {
        let ref_sketch = file_io::fasta_to_sketch(ref_file, k, c, use_syncs);
        ref_sketches.push(ref_sketch);
    }
    let mut query_sketches = vec![];
    if mode == classify {
        let query_file = matches_subc.value_of("query").unwrap();
        query_sketches = file_io::fastq_to_multiple_sketch(query_file, k, c, use_syncs);
    } else {
    }
    println!("Generating sketch time: {}", now.elapsed().as_secs_f32());
    let now = Instant::now();
    if mode == classify {
        for query_sketch in query_sketches.iter() {
            let now = Instant::now();

            for ref_sketch in ref_sketches.iter() {
                chain::chain_seeds(ref_sketch, query_sketch, k, mode);
            }
            println!("Alignment time: {}", now.elapsed().as_secs_f32());
        }
    } else if mode == dist {
        for i in 0..ref_sketches.len() - 1 {
            for j in i + 1..ref_sketches.len() {
                let ref_sketch_i = &ref_sketches[i];
                let ref_sketch_j = &ref_sketches[j];
                chain::chain_seeds(ref_sketch_i, ref_sketch_j, k, mode);
            }
        }
    }
    println!("Alignment time: {}", now.elapsed().as_secs_f32());
}
