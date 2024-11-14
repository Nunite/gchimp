use super::*;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use gchimp::{
    config::{parse_config, Config},
    modules::s2g::S2G,
};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct S2GCliStruct {
    // This is just dummy command because we are already in the command
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    S2G {
        /// Sets path to the target for conversion
        ///
        /// This could be either a .mdl file or a folder for mass conversion
        #[arg(short, long)]
        path: PathBuf,
        /// Skips decompiling (crowbar)
        #[arg(short, long)]
        decompile: bool,
        /// Skips converting .vtf to .png
        #[arg(short, long)]
        vtf: bool,
        /// Skips converting .png to .bmp
        #[arg(short, long)]
        bmp: bool,
        /// Skips converting .qc and .smd
        #[arg(short, long)]
        assembly: bool,
        /// Skips compiling model (studiomdl)
        #[arg(short, long)]
        compile: bool,
        /// Continues with S2G even if there is error
        #[arg(long)]
        force: bool,
        /// WINEPREFIX
        #[arg(long)]
        #[cfg(target_os = "linux")]
        wineprefix: Option<String>,
    },
}

pub struct S2GCli;
impl Cli for S2GCli {
    fn name(&self) -> &'static str {
        "s2g"
    }

    fn cli(&self) -> CliRes {
        let cli = S2GCliStruct::parse();

        let Commands::S2G {
            path,
            decompile,
            vtf,
            bmp,
            assembly,
            compile,
            force,
            #[cfg(target_os = "linux")]
            wineprefix,
        } = cli.command;

        let mut s2g = S2G::new(path.display().to_string().as_str());

        let config = parse_config();

        if config.is_err() {
            println!("Error parsing config.toml");
            return CliRes::Err;
        }

        let Config {
            studiomdl,
            crowbar,
            #[cfg(target_os = "linux")]
                wineprefix: config_wineprefix,
            ..
        } = config.unwrap();

        s2g.decompile(!decompile)
            .vtf(!vtf)
            .bmp(!bmp)
            .smd_and_qc(!assembly)
            .compile(!compile)
            .force(force);

        s2g.studiomdl(PathBuf::from(studiomdl).as_path())
            .crowbar(PathBuf::from(crowbar).as_path());

        #[cfg(target_os = "linux")]
        s2g.wineprefix(if let Some(wineprefix) = &wineprefix {
            wineprefix
        } else {
            config_wineprefix.as_ref().unwrap()
        });

        if let Err(err) = s2g.work() {
            println!("{:?}", err);
            return CliRes::Err;
        };

        CliRes::Ok
    }

    fn cli_help(&self) {
        todo!()
    }
}
