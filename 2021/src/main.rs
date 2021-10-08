use std::fs::{create_dir, File};
use std::io::{BufReader, BufWriter};
use std::iter::once;
use std::path::Path;

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
// "6","1980-01-01 00:00:00","3","en","1209200880","5","5","5","Yes","Yes","No","Yes","No","rustup","5","","5","3","5","Yes","Yes","Yes","Yes","Yes","No","Yes","Yes","No","Yes","Yes","Yes","No","Online download from crates.io","Satisfied with my existing process","Yes","Yes","No","Yes","No"
struct Record {
    #[serde(rename = "Response ID")]
    id: String,
    #[serde(rename = "Date submitted")]
    date: String,
    #[serde(rename = "Last page")]
    page: String,
    #[serde(rename = "Start language")]
    lang: String,
    #[serde(rename = "Seed")]
    seed: String,
    #[serde(rename = "Rust is important to my work or projects (1 disagree - 5 agree)")]
    rust_my_important: String,
    #[serde(
        rename = "Rust will become more important in my work or projects in the future.  (1 disagree - 5 agree)"
    )]
    rust_my_future_important: String,
    #[serde(
        rename = "Rust will become more important to other developers and projects in the future (1 disagree - 5 agree)"
    )]
    rust_other_future_important: String,
    #[serde(rename = "I develop software using Rust")]
    rust_develop: String,
    #[serde(
        rename = "As a developer, I use Rust on the following platforms while programming. Choose all that apply. [MacOS]"
    )]
    rust_develop_mac: String,
    #[serde(
        rename = "As a developer, I use Rust on the following platforms while programming. Choose all that apply. [Windows]"
    )]
    rust_develop_windows: String,
    #[serde(
        rename = "As a developer, I use Rust on the following platforms while programming. Choose all that apply. [Linux (Fedora, OpenSUSE, Ubuntu, Arch, etc)]"
    )]
    rust_develop_linux: String,
    #[serde(
        rename = "As a developer, I use Rust on the following platforms while programming. Choose all that apply. [Other Platform]"
    )]
    rust_develop_other: String,
    #[serde(
        rename = "On your primary development platform, how did you install your Rust toolchain?"
    )]
    rust_develop_toolchain_install: String,
    #[serde(
        rename = "The following features or tools are important in my development environment (do not use 1 - use a lot 5).  [Integrated Development Environments with Language Features (syntax highlight, errors, completion, type checking)]"
    )]
    rust_develop_feature_ide: String,
    #[serde(
        rename = "The following features or tools are important in my development environment (do not use 1 - use a lot 5).  [Debugging tools (lldb, gdb)]"
    )]
    rust_develop_feature_debugger: String,
    #[serde(
        rename = "The following features or tools are important in my development environment (do not use 1 - use a lot 5).  [Online Documentation (doc.rust-lang.org, docs.rs)]"
    )]
    rust_develop_feature_online_doc: String,
    #[serde(
        rename = "The following features or tools are important in my development environment (do not use 1 - use a lot 5).  [Offline Documentation (local) ]"
    )]
    rust_develop_feature_offline_doc: String,
    #[serde(
        rename = "The following features or tools are important in my development environment (do not use 1 - use a lot 5).  [Build Caching (sccache)]"
    )]
    rust_develop_feature_sccache: String,
    #[serde(rename = "Do you run, provide, distribute or deploy Rust software?")]
    rust_provides: String,
    #[serde(
        rename = "Which platforms (operating systems) do you target for Rust software. Check all that apply. [Linux]"
    )]
    rust_provides_target_linux: String,
    #[serde(
        rename = "Which platforms (operating systems) do you target for Rust software. Check all that apply. [MacOS]"
    )]
    rust_provides_target_macos: String,
    #[serde(
        rename = "Which platforms (operating systems) do you target for Rust software. Check all that apply. [Windows]"
    )]
    rust_provides_target_windows: String,
    #[serde(
        rename = "Which platforms (operating systems) do you target for Rust software. Check all that apply. [Rust Library - No specific target (crates.io)]"
    )]
    rust_provides_target_library: String,
    #[serde(
        rename = "Which platforms (operating systems) do you target for Rust software. Check all that apply. [Other]"
    )]
    rust_provides_target_other: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Local use only (cargo build / cargo run)]"
    )]
    rust_provides_env_local: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Containers, using rustup inside the builder]"
    )]
    rust_provides_env_container_rustup: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Containers, using a packaged rust (dnf, zypper, apt, pacman, etc)]"
    )]
    rust_provides_env_container_pkgrust: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Packaged for a package manager (rpm, deb, brew, msi, pkg, dmg, etc)]"
    )]
    rust_provides_env_packaged: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Provided as a library (source, crates.io)]"
    )]
    rust_provides_env_library: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Other Process]"
    )]
    rust_provides_env_other: String,
    #[serde(
        rename = "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question. [Unknown]"
    )]
    rust_provides_env_unknown: String,
    #[serde(rename = "In your release process, how do you manage your Rust dependencies?")]
    rust_provides_dependency: String,
    #[serde(
        rename = "In your ideal workflow, how would you prefer to manager your Rust dependencies? "
    )]
    rust_provides_ideal_dependency: String,
    #[serde(
        rename = "How do you manage security updates in your Rust dependencies? [Security scanners (e.g. cargo audit)]"
    )]
    rust_provides_security_audit: String,
    #[serde(
        rename = "How do you manage security updates in your Rust dependencies? [Frequent update of all dependencies (e.g. cargo outdated)]"
    )]
    rust_provides_security_outdated: String,
    #[serde(
        rename = "How do you manage security updates in your Rust dependencies? [Don't actively follow security issues in dependencies]"
    )]
    rust_provides_security_noaction: String,
    #[serde(
        rename = "How do you manage security updates in your Rust dependencies? [Rely on external source to manage (ie distribution packaged rust libraries)]"
    )]
    rust_provides_security_pkgmaint: String,
    #[serde(rename = "How do you manage security updates in your Rust dependencies? [Unknown]")]
    rust_provides_security_unknown: String,
}

fn map_to_rows(q: &str, sz: usize, map: BTreeMap<String, u32>) -> (Vec<String>, Vec<String>) {
    once((q.to_string(), sz.to_string()))
        .chain(map.into_iter().map(|(k, v)| (k, v.to_string())))
        .unzip()
}

fn write_csv(hdr: &[String], row: &[String], path: &str) {
    let f = File::create(path).expect("Unable to create ./output/rust_my_important.csv");
    let mut wtr = csv::Writer::from_writer(BufWriter::new(f));
    wtr.write_record(hdr).unwrap();
    wtr.write_record(row).unwrap();
    wtr.flush().unwrap();
}

fn main() {
    let f = File::open("./results-survey417611.csv").expect("Unable to open file");
    let reader = BufReader::new(f);

    // Setup the output dir if needed.
    let outdir = Path::new("./output");
    if !outdir.exists() {
        create_dir(outdir).expect("Unable to create output dir");
    };

    let mut rdr = csv::Reader::from_reader(reader);
    let mut data = Vec::with_capacity(1400);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result.unwrap();
        // println!("{:?}", record);
        data.push(record);
    }

    // Data normalisation - this is where we cleanup things that well ... need cleaning!
    data.iter_mut().for_each(|rec| {
        // rust_my_important - no response is "", we want "n/a"
        if rec.rust_my_important == "" {
            rec.rust_my_important = "n/a".to_string();
        }
        if rec.rust_my_future_important == "" {
            rec.rust_my_future_important = "n/a".to_string();
        }
        if rec.rust_other_future_important == "" {
            rec.rust_other_future_important = "n/a".to_string();
        }
        // If they are satisfied with existing, then we move that pref to ideal.
        if rec.rust_provides_ideal_dependency == "Satisfied with my existing process" {
            rec.rust_provides_ideal_dependency = rec.rust_provides_dependency.clone()
        }
    });

    // Outputs -
    //   sums of
    //     * rust_my_important

    let mut my_important_map: BTreeMap<String, u32> = BTreeMap::new();
    data.iter().for_each(|rec| {
        if let Some(cnt) = my_important_map.get_mut(&rec.rust_my_important) {
            *cnt += 1;
        } else {
            my_important_map.insert(rec.rust_my_important.clone(), 1);
        }
    });

    let (hdr, row) = map_to_rows(
        "Rust is important to my work or projects (1 disagree - 5 agree)",
        data.len(),
        my_important_map,
    );
    write_csv(&hdr, &row, "./output/01_rust_my_important.csv");

    //     * rust_my_future_important
    let mut my_future_important_map: BTreeMap<String, u32> = BTreeMap::new();
    data.iter().for_each(|rec| {
        if let Some(cnt) = my_future_important_map.get_mut(&rec.rust_my_future_important) {
            *cnt += 1;
        } else {
            my_future_important_map.insert(rec.rust_my_future_important.clone(), 1);
        }
    });

    let (hdr, row) = map_to_rows(
        "Rust will become more important in my work or projects in the future.  (1 disagree - 5 agree)",
        data.len(),
        my_future_important_map,
    );
    write_csv(&hdr, &row, "./output/02_rust_my_future_important.csv");

    //     * rust_other_future_important
    let mut other_important_map: BTreeMap<String, u32> = BTreeMap::new();
    data.iter().for_each(|rec| {
        if let Some(cnt) = other_important_map.get_mut(&rec.rust_other_future_important) {
            *cnt += 1;
        } else {
            other_important_map.insert(rec.rust_other_future_important.clone(), 1);
        }
    });

    let (hdr, row) = map_to_rows(
        "Rust will become more important to other developers and projects in the future (1 disagree - 5 agree)",
        data.len(),
        other_important_map,
    );
    write_csv(&hdr, &row, "./output/03_rust_other_future_important.csv");

    //     * rust_develop_*

    // rust_develop_mac, rust_develop_windows, rust_develop_linux, rust_develop_other
    let mut dev_plat_mac: u32 = 0;
    let mut dev_plat_win: u32 = 0;
    let mut dev_plat_lnx: u32 = 0;
    let mut dev_plat_lnx_only: u32 = 0;
    let mut dev_plat_oth: u32 = 0;
    let mut count = 0;

    data.iter().for_each(|rec| {
        // println!("{}, {}, {}, {}", rec.rust_develop_mac, rec.rust_develop_windows, rec.rust_develop_linux, rec.rust_develop_other);
        let mut inc = false;
        if rec.rust_develop_mac == "Yes" {
            inc = true;
            dev_plat_mac += 1;
        }
        if rec.rust_develop_windows == "Yes" {
            inc = true;
            dev_plat_win += 1;
        }
        if rec.rust_develop_linux == "Yes" {
            inc = true;
            dev_plat_lnx += 1;
        }
        if rec.rust_develop_other == "Yes" {
            inc = true;
            dev_plat_oth += 1;
        }

        if rec.rust_develop_linux == "Yes"
            && rec.rust_develop_mac == "No"
            && rec.rust_develop_windows == "No"
            && rec.rust_develop_other == "No"
        {
            dev_plat_lnx_only += 1;
        }

        if inc {
            count += 1;
        }
    });

    let hdr = vec![
        "As a developer, I use Rust on the following platforms while programming.".to_string(),
        "Mac".to_string(),
        "Windows".to_string(),
        "Linux".to_string(),
        "Linux Only".to_string(),
        "Other".to_string(),
    ];
    let row = vec![
        count.to_string(),
        dev_plat_mac.to_string(),
        dev_plat_win.to_string(),
        dev_plat_lnx.to_string(),
        dev_plat_lnx_only.to_string(),
        dev_plat_oth.to_string(),
    ];

    write_csv(&hdr, &row, "./output/04_rust_develop_platform.csv");

    //     * rust_develop_toolchain_install
    let mut develop_toolchain_install: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_toolchain_install != "" {
            sz += 1;
            if let Some(cnt) =
                develop_toolchain_install.get_mut(&rec.rust_develop_toolchain_install)
            {
                *cnt += 1;
            } else {
                develop_toolchain_install.insert(rec.rust_develop_toolchain_install.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "On your primary development platform, how did you install your Rust toolchain?",
        sz,
        develop_toolchain_install,
    );
    write_csv(&hdr, &row, "./output/05_rust_develop_toolchain_install.csv");

    //     * rust_develop_feature_ide
    let mut develop_feature_ide: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_ide != "" {
            sz += 1;
            if let Some(cnt) = develop_feature_ide.get_mut(&rec.rust_develop_feature_ide) {
                *cnt += 1;
            } else {
                develop_feature_ide.insert(rec.rust_develop_feature_ide.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "Integrated Development Environments with Language Features (syntax highlight, errors, completion, type checking",
        sz,
        develop_feature_ide,
    );
    write_csv(&hdr, &row, "./output/06_rust_develop_feature_ide.csv");

    //     * rust_develop_feature_debugger
    let mut develop_feature_debugger: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_debugger != "" {
            sz += 1;
            if let Some(cnt) = develop_feature_debugger.get_mut(&rec.rust_develop_feature_debugger)
            {
                *cnt += 1;
            } else {
                develop_feature_debugger.insert(rec.rust_develop_feature_debugger.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows("Debugging tools (lldb, gdb)", sz, develop_feature_debugger);
    write_csv(&hdr, &row, "./output/07_rust_develop_feature_debugger.csv");

    //     * rust_develop_feature_online_doc
    let mut develop_feature_online_doc: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_online_doc != "" {
            sz += 1;
            if let Some(cnt) =
                develop_feature_online_doc.get_mut(&rec.rust_develop_feature_online_doc)
            {
                *cnt += 1;
            } else {
                develop_feature_online_doc.insert(rec.rust_develop_feature_online_doc.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "Online Documentation (doc.rust-lang.org, docs.rs)",
        sz,
        develop_feature_online_doc,
    );
    write_csv(
        &hdr,
        &row,
        "./output/08_rust_develop_feature_online_doc.csv",
    );

    //     * rust_develop_feature_offline_doc
    let mut develop_feature_offline_doc: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_offline_doc != "" {
            sz += 1;
            if let Some(cnt) =
                develop_feature_offline_doc.get_mut(&rec.rust_develop_feature_offline_doc)
            {
                *cnt += 1;
            } else {
                develop_feature_offline_doc.insert(rec.rust_develop_feature_offline_doc.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "Offline Documentation (local)",
        sz,
        develop_feature_offline_doc,
    );
    write_csv(
        &hdr,
        &row,
        "./output/09_rust_develop_feature_offline_doc.csv",
    );

    //     * rust_develop_feature_sccache
    let mut develop_feature_sccache: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_sccache != "" {
            sz += 1;
            if let Some(cnt) = develop_feature_sccache.get_mut(&rec.rust_develop_feature_sccache) {
                *cnt += 1;
            } else {
                develop_feature_sccache.insert(rec.rust_develop_feature_sccache.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows("Build Caching (sccache)", sz, develop_feature_sccache);
    write_csv(&hdr, &row, "./output/10_rust_develop_feature_sccache.csv");

    //     * rust_provides_target_*
    let mut tgt_plat_mac: u32 = 0;
    let mut tgt_plat_win: u32 = 0;
    let mut tgt_plat_lnx: u32 = 0;
    let mut tgt_plat_lib: u32 = 0;
    let mut tgt_plat_oth: u32 = 0;
    let mut count = 0;

    data.iter().for_each(|rec| {
        let mut inc = false;
        if rec.rust_provides_target_macos == "Yes" {
            inc = true;
            tgt_plat_mac += 1;
        }
        if rec.rust_provides_target_windows == "Yes" {
            inc = true;
            tgt_plat_win += 1;
        }
        if rec.rust_provides_target_linux == "Yes" {
            inc = true;
            tgt_plat_lnx += 1;
        }
        if rec.rust_provides_target_library == "Yes" {
            inc = true;
            tgt_plat_lib += 1;
        }
        if rec.rust_provides_target_other == "Yes" {
            inc = true;
            tgt_plat_oth += 1;
        }
        if inc {
            count += 1;
        }
    });

    let hdr = vec![
        "Which platforms (operating systems) do you target for Rust software".to_string(),
        "Mac".to_string(),
        "Windows".to_string(),
        "Linux".to_string(),
        "Library".to_string(),
        "Other".to_string(),
    ];
    let row = vec![
        count.to_string(),
        tgt_plat_mac.to_string(),
        tgt_plat_win.to_string(),
        tgt_plat_lnx.to_string(),
        tgt_plat_lib.to_string(),
        tgt_plat_oth.to_string(),
    ];

    write_csv(&hdr, &row, "./output/11_rust_target_platform.csv");

    //     * rust_provides_env_*
    let mut env_plat_lcl: u32 = 0;
    let mut env_plat_cru: u32 = 0;
    let mut env_plat_crp: u32 = 0;
    let mut env_plat_pkg: u32 = 0;
    let mut env_plat_lib: u32 = 0;
    let mut env_plat_oth: u32 = 0;
    let mut env_plat_unk: u32 = 0;
    let mut count = 0;

    data.iter().for_each(|rec| {
        let mut inc = false;
        if rec.rust_provides_env_local == "Yes" {
            inc = true;
            env_plat_lcl += 1;
        }
        if rec.rust_provides_env_container_rustup == "Yes" {
            inc = true;
            env_plat_cru += 1;
        }
        if rec.rust_provides_env_container_pkgrust == "Yes" {
            inc = true;
            env_plat_crp += 1;
        }
        if rec.rust_provides_env_packaged == "Yes" {
            inc = true;
            env_plat_pkg += 1;
        }
        if rec.rust_provides_env_library == "Yes" {
            inc = true;
            env_plat_lib += 1;
        }
        if rec.rust_provides_env_other == "Yes" {
            inc = true;
            env_plat_oth += 1;
        }
        if rec.rust_provides_env_unknown == "Yes" {
            inc = true;
            env_plat_unk += 1;
        }
        if inc {
            count += 1;
        }
    });
    let hdr = vec![
        "How do you or your team/community build or provide Rust software for people to use? Think about your build pipeline or deployment processes for this question.".to_string(),
        "Local".to_string(),
        "Container - rustup".to_string(),
        "Container - packaged rust".to_string(),
        "Packaged".to_string(),
        "Library".to_string(),
        "Other".to_string(),
        "Unknown".to_string(),
    ];
    let row = vec![
        count.to_string(),
        env_plat_lcl.to_string(),
        env_plat_cru.to_string(),
        env_plat_crp.to_string(),
        env_plat_pkg.to_string(),
        env_plat_lib.to_string(),
        env_plat_oth.to_string(),
        env_plat_unk.to_string(),
    ];

    write_csv(&hdr, &row, "./output/12_rust_target_env.csv");

    //     * rust_provides_dependency
    let mut provides_dependency: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_provides_dependency != "" {
            sz += 1;
            if let Some(cnt) = provides_dependency.get_mut(&rec.rust_provides_dependency) {
                *cnt += 1;
            } else {
                provides_dependency.insert(rec.rust_provides_dependency.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "In your release process, how do you manage your Rust dependencies?",
        sz,
        provides_dependency,
    );
    write_csv(&hdr, &row, "./output/13_rust_provides_dependency.csv");

    //     * rust_provides_ideal_dependency
    let mut provides_ideal_dependency: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_provides_ideal_dependency != "" {
            sz += 1;
            if let Some(cnt) =
                provides_ideal_dependency.get_mut(&rec.rust_provides_ideal_dependency)
            {
                *cnt += 1;
            } else {
                provides_ideal_dependency.insert(rec.rust_provides_ideal_dependency.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "In your ideal workflow, how would you prefer to manager your Rust dependencies?",
        sz,
        provides_ideal_dependency,
    );
    write_csv(&hdr, &row, "./output/14_rust_provides_ideal_dependency.csv");

    //     * rust_provides_security_*
    //  15
    let mut sec_mgmt_aud: u32 = 0;
    let mut sec_mgmt_out: u32 = 0;
    let mut sec_mgmt_noa: u32 = 0;
    let mut sec_mgmt_pkg: u32 = 0;
    let mut sec_mgmt_unk: u32 = 0;
    let mut count = 0;

    data.iter().for_each(|rec| {
        let mut inc = false;
        if rec.rust_provides_security_audit == "Yes" {
            inc = true;
            sec_mgmt_aud += 1;
        }
        if rec.rust_provides_security_outdated == "Yes" {
            inc = true;
            sec_mgmt_out += 1;
        }
        if rec.rust_provides_security_noaction == "Yes" {
            inc = true;
            sec_mgmt_noa += 1;
        }
        if rec.rust_provides_security_pkgmaint == "Yes" {
            inc = true;
            sec_mgmt_pkg += 1;
        }
        if rec.rust_provides_security_unknown == "Yes" {
            inc = true;
            sec_mgmt_unk += 1;
        }
        if inc {
            count += 1;
        }
    });
    let hdr = vec![
        "How do you manage security updates in your Rust dependencies?".to_string(),
        "cargo audit".to_string(),
        "cargo outdated".to_string(),
        "Don't actively follow".to_string(),
        "Distribution pkg libs".to_string(),
        "Unknown".to_string(),
    ];
    let row = vec![
        count.to_string(),
        sec_mgmt_aud.to_string(),
        sec_mgmt_out.to_string(),
        sec_mgmt_noa.to_string(),
        sec_mgmt_pkg.to_string(),
        sec_mgmt_unk.to_string(),
    ];

    write_csv(&hdr, &row, "./output/15_rust_provides_security.csv");

    // correlations
    //  - What do packagers want?
    let mut provides_ideal_dependency: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_provides_ideal_dependency != "" && rec.rust_provides_env_packaged == "Yes" {
            sz += 1;
            if let Some(cnt) =
                provides_ideal_dependency.get_mut(&rec.rust_provides_ideal_dependency)
            {
                *cnt += 1;
            } else {
                provides_ideal_dependency.insert(rec.rust_provides_ideal_dependency.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Provides Packages) In your ideal workflow, how would you prefer to manager your Rust dependencies?",
        sz,
        provides_ideal_dependency,
    );
    write_csv(
        &hdr,
        &row,
        "./output/16_rust_packages_provides_ideal_dependency.csv",
    );

    // - What containers want
    let mut provides_ideal_dependency: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_provides_ideal_dependency != ""
            && (rec.rust_provides_env_container_rustup == "Yes"
                || rec.rust_provides_env_container_pkgrust == "Yes")
        {
            sz += 1;
            if let Some(cnt) =
                provides_ideal_dependency.get_mut(&rec.rust_provides_ideal_dependency)
            {
                *cnt += 1;
            } else {
                provides_ideal_dependency.insert(rec.rust_provides_ideal_dependency.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Provides Containers) In your ideal workflow, how would you prefer to manager your Rust dependencies?",
        sz,
        provides_ideal_dependency,
    );
    write_csv(
        &hdr,
        &row,
        "./output/17_rust_containers_provides_ideal_dependency.csv",
    );

    //  - dev plat to how they install

    let mut develop_toolchain_install: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_toolchain_install != "" && rec.rust_develop_linux == "Yes" {
            sz += 1;
            if let Some(cnt) =
                develop_toolchain_install.get_mut(&rec.rust_develop_toolchain_install)
            {
                *cnt += 1;
            } else {
                develop_toolchain_install.insert(rec.rust_develop_toolchain_install.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Linux Developers) On your primary development platform, how did you install your Rust toolchain?",
        sz,
        develop_toolchain_install,
    );
    write_csv(
        &hdr,
        &row,
        "./output/18_rust_linux_develop_toolchain_install.csv",
    );

    let mut develop_toolchain_install: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_toolchain_install != ""
            && rec.rust_develop_linux == "Yes"
            && rec.rust_develop_mac == "No"
            && rec.rust_develop_windows == "No"
            && rec.rust_develop_other == "No"
        {
            sz += 1;
            if let Some(cnt) =
                develop_toolchain_install.get_mut(&rec.rust_develop_toolchain_install)
            {
                *cnt += 1;
            } else {
                develop_toolchain_install.insert(rec.rust_develop_toolchain_install.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Linux Only Developers) On your primary development platform, how did you install your Rust toolchain?",
        sz,
        develop_toolchain_install,
    );
    write_csv(
        &hdr,
        &row,
        "./output/18_rust_linux_only_develop_toolchain_install.csv",
    );

    let mut develop_toolchain_install: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_toolchain_install != "" && rec.rust_develop_linux == "No" {
            sz += 1;
            if let Some(cnt) =
                develop_toolchain_install.get_mut(&rec.rust_develop_toolchain_install)
            {
                *cnt += 1;
            } else {
                develop_toolchain_install.insert(rec.rust_develop_toolchain_install.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Non-Linux Developers) On your primary development platform, how did you install your Rust toolchain?",
        sz,
        develop_toolchain_install,
    );
    write_csv(
        &hdr,
        &row,
        "./output/19_rust_nonlinux_develop_toolchain_install.csv",
    );

    // People who used packaged rust, features.
    let mut develop_feature_ide: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_ide != ""
            && rec.rust_develop_toolchain_install == "Package Manager (dnf, apt, brew, zypper, etc)"
            && rec.rust_develop_linux == "Yes"
        {
            sz += 1;
            if let Some(cnt) = develop_feature_ide.get_mut(&rec.rust_develop_feature_ide) {
                *cnt += 1;
            } else {
                develop_feature_ide.insert(rec.rust_develop_feature_ide.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Linux Packaged) Integrated Development Environments with Language Features (syntax highlight, errors, completion, type checking",
        sz,
        develop_feature_ide,
    );
    write_csv(
        &hdr,
        &row,
        "./output/20_rust_packaged_develop_feature_ide.csv",
    );

    //     * rust_develop_feature_debugger
    let mut develop_feature_debugger: BTreeMap<String, u32> = BTreeMap::new();
    let mut sz: usize = 0;
    data.iter().for_each(|rec| {
        if rec.rust_develop_feature_debugger != ""
            && rec.rust_develop_toolchain_install == "Package Manager (dnf, apt, brew, zypper, etc)"
            && rec.rust_develop_linux == "Yes"
        {
            sz += 1;
            if let Some(cnt) = develop_feature_debugger.get_mut(&rec.rust_develop_feature_debugger)
            {
                *cnt += 1;
            } else {
                develop_feature_debugger.insert(rec.rust_develop_feature_debugger.clone(), 1);
            }
        }
    });

    let (hdr, row) = map_to_rows(
        "(Linux Packaged) Debugging tools (lldb, gdb)",
        sz,
        develop_feature_debugger,
    );
    write_csv(
        &hdr,
        &row,
        "./output/21_rust_packaged_develop_feature_debugger.csv",
    );
}
