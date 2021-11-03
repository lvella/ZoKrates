use crate::constants::{FLATTENED_CODE_DEFAULT_PATH, MPC_DEFAULT_PATH};
use clap::{App, Arg, ArgMatches, SubCommand};
use phase2::parameters::MPCParameters;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zokrates_core::ir;
use zokrates_core::ir::ProgEnum;
use zokrates_core::proof_system::bellman::Computation;
use zokrates_field::Bn128Field;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("verify")
        .about("Verify the correctness of the MPC parameters, given a circuit instance")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Path of the binary")
                .value_name("FILE")
                .takes_value(true)
                .required(false)
                .default_value(FLATTENED_CODE_DEFAULT_PATH),
        )
        .arg(
            Arg::with_name("mpc-params")
                .short("p")
                .long("mpc-params")
                .help("Path of the MPC parameters")
                .value_name("FILE")
                .takes_value(true)
                .required(false)
                .default_value(MPC_DEFAULT_PATH),
        )
        .arg(
            Arg::with_name("radix-dir")
                .short("r")
                .long("radix-dir")
                .help("Path of the directory containing parameters for various 2^m circuit depths (phase1radix2m{0..=m})")
                .value_name("PATH")
                .takes_value(true)
                .required(true),
        )
}

pub fn exec(sub_matches: &ArgMatches) -> Result<(), String> {
    // read compiled program
    let path = Path::new(sub_matches.value_of("input").unwrap());
    let file =
        File::open(&path).map_err(|why| format!("Could not open `{}`: {}", path.display(), why))?;

    let mut reader = BufReader::new(file);

    match ProgEnum::deserialize(&mut reader)? {
        ProgEnum::Bn128Program(p) => cli_mpc_verify(p, sub_matches),
        _ => unimplemented!(),
    }
}

fn cli_mpc_verify(ir_prog: ir::Prog<Bn128Field>, sub_matches: &ArgMatches) -> Result<(), String> {
    println!("Verifying contributions...");

    let path = Path::new(sub_matches.value_of("mpc-params").unwrap());
    let file =
        File::open(&path).map_err(|why| format!("Could not open `{}`: {}", path.display(), why))?;

    let reader = BufReader::new(file);
    let params = MPCParameters::read(reader, true)
        .map_err(|why| format!("Could not read `{}`: {}", path.display(), why))?;

    let radix_dir = Path::new(sub_matches.value_of("radix-dir").unwrap());
    let circuit = Computation::without_witness(ir_prog);

    let result = params
        .verify(
            circuit,
            true,
            &radix_dir
                .to_path_buf()
                .into_os_string()
                .into_string()
                .unwrap(),
        )
        .map_err(|_| "Verification failed".to_string())?;

    println!("\nContributions:");

    for (i, hash) in result.iter().enumerate() {
        println!("{}: {}", i, hex::encode(hash));
    }

    println!("\nContributions verified");
    Ok(())
}