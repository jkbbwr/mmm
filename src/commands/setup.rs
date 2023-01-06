use std::io::prelude::*;

use std::{fs::File, path::Path};

use argh::FromArgs;
use axum::async_trait;
use chrono::{Local, Months};
use eyre::{bail, eyre};
use openssl::asn1::Asn1Time;

use openssl::hash::MessageDigest;
use openssl::pkey;
use openssl::x509::{self, X509Name};

use crate::settings::Settings;

use super::Run;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "setup", description = "Setup mmm.")]
pub struct Setup {
    #[argh(switch, description = "overwrite the existing config file.")]
    overwrite: bool,

    #[argh(
        option,
        description = "path for the ca private key.",
        default = "String::from(\"./keys/ca.private\")"
    )]
    ca_private_key_path: String,

    #[argh(
        option,
        description = "path for the ca cert.",
        default = "String::from(\"./certs/ca_root.cert\")"
    )]
    ca_root_certificate_path: String,

    #[argh(
        option,
        description = "ca private rsa key size in bits",
        default = "4096"
    )]
    ca_private_key_bit_size: u32,

    #[argh(
        option,
        description = "x509 country code",
        default = "String::from(\"uk\")"
    )]
    x509_c: String,

    #[argh(
        option,
        description = "x509 state",
        default = "String::from(\"england\")"
    )]
    x509_st: String,

    #[argh(option, description = "x509 org", default = "String::from(\"mmm\")")]
    x509_o: String,

    #[argh(
        option,
        description = "x509 common name",
        default = "String::from(\"mmm.self.signed\")"
    )]
    x509_cn: String,
}

#[async_trait]
impl Run for Setup {
    async fn run(&self, _settings: &Settings) -> eyre::Result<()> {
        let mut settings = Settings::default();

        settings.keys.ca_private_key_path = self.ca_private_key_path.clone();
        settings.keys.ca_root_certificate_path = self.ca_root_certificate_path.clone();

        if Path::new("./mmm.toml").exists() && !self.overwrite {
            bail!("./mmm.toml exists and --overwrite wasn't given.");
        }

        bunt::println!(
            "{$green}Generated the following config{/$}\n{:#?}",
            settings
        );

        bunt::println!("{$green}Writing the config file to ./mmm.toml{/$}");
        let mut file = File::create("./mmm.toml")?;
        write!(file, "{}", toml::to_string(&settings)?)?;

        bunt::println!("{$green}Generating ca private key{/$}");

        let ca_private_key = openssl::rsa::Rsa::generate(self.ca_private_key_bit_size)?;

        if Path::new(&self.ca_private_key_path).exists() && !self.overwrite {
            bail!(
                "{} exists and --overwrite wasn't given.",
                self.ca_private_key_path
            );
        }

        bunt::println!(
            "{$green}Generating RSA private key of size {}{/$}",
            self.ca_private_key_bit_size
        );

        bunt::println!("{$green}Saving to {}{/$}", self.ca_private_key_path);
        let mut ca_private_key_file = File::create(&self.ca_private_key_path)?;
        ca_private_key_file.write_all(&ca_private_key.private_key_to_pem()?)?;

        bunt::println!("{$green}Generating CA root cert.{/$}");

        let mut ca_builder = x509::X509::builder()?;
        ca_builder.set_version(1)?;

        let now = Asn1Time::from_unix(Local::now().timestamp())?;
        ca_builder.set_not_before(&now)?;

        let years_time = Asn1Time::from_unix(
            Local::now()
                .checked_add_months(Months::new(12))
                .ok_or(eyre!("Failed to add 12 months"))?
                .timestamp(),
        )?;
        ca_builder.set_not_after(&years_time)?;

        let mut name_builder = X509Name::builder()?;
        name_builder.append_entry_by_text("C", &self.x509_c)?;
        name_builder.append_entry_by_text("ST", &self.x509_st)?;
        name_builder.append_entry_by_text("O", &self.x509_o)?;
        name_builder.append_entry_by_text("CN", &self.x509_cn)?;

        let name = name_builder.build();
        ca_builder.set_issuer_name(&name)?;
        ca_builder.set_subject_name(&name)?;

        let private_key = pkey::PKey::from_rsa(ca_private_key)?;

        ca_builder.set_pubkey(&private_key)?;

        ca_builder.sign(
            &private_key,
            MessageDigest::from_name("sha256").ok_or(eyre!("Can't find sha256"))?,
        )?;

        let ca_cert = ca_builder.build();

        bunt::println!("{$green}Saving to {}{/$}", self.ca_root_certificate_path);
        let mut ca_cert_file = File::create(&self.ca_root_certificate_path)?;
        ca_cert_file.write_all(&ca_cert.to_pem()?)?;

        Ok(())
    }
}
