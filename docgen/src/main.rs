use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{env, fs};
use zqutils::commands::CommandBuilder;

const API_DIRNAME: &str = "api";

#[derive(Clone, Deserialize)]
struct Version {
    refspec: String,
    name: Option<String>,
    api_url: String,
}

#[derive(Clone, Deserialize)]
struct Zq2Spec {
    versions: Vec<Version>,
}

/// Turns out we want to reorder navigation paths. We do this by including dummy
/// entries in the navigation; we then read in the sequence in nav, and reorder it
/// appropiately
async fn reorder_top_level_navigation(for_index_file: &Path) -> Result<()> {
    // Read `for_index_file`. Now, for each prefix in `api_prefixes` (dot-separated),
    // add an initial component of `add_component` to their referenced paths.
    let contents = fs::read_to_string(for_index_file)?;
    let mut value_pre: serde_yaml::Value = serde_yaml::from_str(&contents)?;
    if let serde_yaml::Value::Mapping(top_level_map) = &mut value_pre {
        // Find "nav".
        if let Some(serde_yaml::Value::Sequence(top_level_seq)) =
            top_level_map.get(serde_yaml::Value::String("nav".to_string()))
        {
            // OK. We're going to change this sequence. Construct a new sequence ...
            // Again, O(n^2), I'm afraid.
            let mut replacement_sequence = serde_yaml::Sequence::new();
            let mut do_not_copy_elements: HashSet<String> = HashSet::new();
            let mut elements_by_name: HashMap<String, serde_yaml::Value> = HashMap::new();

            for seq_elem in top_level_seq {
                //println!("SE {seq_elem:?}");
                if let serde_yaml::Value::Mapping(inner_map) = seq_elem {
                    for k in inner_map.keys() {
                        if let serde_yaml::Value::String(key) = k {
                            elements_by_name.insert(
                                key.to_string(),
                                serde_yaml::Value::Mapping(inner_map.clone()),
                            );
                        }
                    }
                }
                if let serde_yaml::Value::String(key) = seq_elem {
                    //println!("FFF Found {key}");
                    if let Some(rest) = key.strip_prefix('$') {
                        // It's a substitution key.
                        do_not_copy_elements.insert(rest.to_string());
                    }
                }
            }

            //println!("do_not_copy {do_not_copy_elements:?} elements_by_name {elements_by_name:?}");

            for seq_elem in top_level_seq {
                let mut copy_key = true;
                match seq_elem {
                    serde_yaml::Value::Mapping(inner_map) => {
                        for k in inner_map.keys() {
                            if let serde_yaml::Value::String(key) = k {
                                if do_not_copy_elements.contains(key) {
                                    copy_key = false;
                                    break;
                                }
                            }
                        }
                        if copy_key {
                            replacement_sequence.push(seq_elem.clone());
                        }
                    }
                    serde_yaml::Value::String(key) => {
                        if let Some(rest) = key.strip_prefix('$') {
                            //println!("QQQQ {rest}");
                            if let Some(v) = elements_by_name.get(rest) {
                                replacement_sequence.push(v.clone());
                            }
                        } else {
                            replacement_sequence.push(seq_elem.clone());
                        }
                    }
                    _ => replacement_sequence.push(seq_elem.clone()),
                }
            }
            //println!("PPPP {replacement_sequence:?}");
            top_level_map.insert(
                serde_yaml::Value::String("nav".to_string()),
                serde_yaml::Value::Sequence(replacement_sequence),
            );
        }
    };
    let result = serde_yaml::to_string(&value_pre)?;
    fs::write(for_index_file, result.as_bytes())?;
    Ok(())
}

/// There is an annoying bug in docgen in the zq2 code which fixes id-prefix and
/// nav-prefix to be the same - you can't have things in the nav heirarchy which
/// have more file components than they have id components.
/// But of course, we want to. And so we postprocess mkdocs.yaml to synthetically
/// add those prefixes for the paths we care about.
async fn fixup_api_paths(
    for_index_file: &Path,
    api_prefixes: &HashSet<String>,
    add_component: &str,
) -> Result<()> {
    // This is positively the worst way to do this, but it is the quickest, and
    // N is small.
    fn process_value(
        prefix: &Option<String>,
        api_prefixes: &HashSet<String>,
        add_component: &str,
        value: &serde_yaml::Value,
        triggered_in: bool,
    ) -> Result<serde_yaml::Value> {
        let mut triggered_out = triggered_in;
        if let Some(v) = &prefix {
            if api_prefixes.contains(v) {
                triggered_out = true;
            }
        }
        match value {
            serde_yaml::Value::Sequence(seq) => {
                // Just iterate.
                let mut new_seq = serde_yaml::value::Sequence::new();
                for elem in seq {
                    new_seq.push(process_value(
                        prefix,
                        api_prefixes,
                        add_component,
                        elem,
                        triggered_out,
                    )?);
                }
                Ok(serde_yaml::Value::Sequence(new_seq))
            }
            serde_yaml::Value::Mapping(map) => {
                let mut new_map = serde_yaml::value::Mapping::new();
                for (key_val, val) in map {
                    if let serde_yaml::Value::String(key) = key_val {
                        let new_key = key.to_string();
                        let new_prefix = Some(if let Some(v) = prefix {
                            format!("{v}/{key}")
                        } else {
                            key.to_string()
                        });
                        new_map.insert(
                            serde_yaml::Value::String(new_key),
                            process_value(
                                &new_prefix,
                                api_prefixes,
                                add_component,
                                val,
                                triggered_out,
                            )?,
                        );
                    } else {
                        new_map.insert(
                            key_val.clone(),
                            process_value(prefix, api_prefixes, add_component, val, triggered_out)?,
                        );
                    }
                }
                Ok(serde_yaml::Value::Mapping(new_map))
            }
            serde_yaml::Value::String(s) => {
                if triggered_out {
                    // The starts_with check protects us from upstream fixes.
                    let new_s = if !s.starts_with(add_component) {
                        format!("{}{}", add_component, s)
                    } else {
                        s.to_string()
                    };
                    Ok(serde_yaml::Value::String(new_s))
                } else {
                    Ok(serde_yaml::Value::String(s.to_string()))
                }
            }
            _ => Ok(value.clone()),
        }
    }

    // Read `for_index_file`. Now, for each prefix in `api_prefixes` (dot-separated),
    // add an initial component of `add_component` to their referenced paths.
    let contents = fs::read_to_string(for_index_file)?;
    let value_pre: serde_yaml::Value = serde_yaml::from_str(&contents)?;
    let value = process_value(&None, api_prefixes, add_component, &value_pre, false)?;
    let result = serde_yaml::to_string(&value)?;
    fs::write(for_index_file, result.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let here = String::from(&args[1]);
    let mut api_prefixes: HashSet<String> = HashSet::new();

    // Set NO_CHECKOUT to skip the checkout steps - this allows you to do debugging with symlinks or similar.
    let mut checkout = std::env::var("NO_CHECKOUT").is_err();

    // Find the zq2 versions that we need to collect.
    let root_path = PathBuf::from(&here);
    let versions: Zq2Spec =
        serde_yaml::from_str(&fs::read_to_string(format!("{}/zq2_spec.yaml", here))?)?;
    let overridden_zq2 = std::env::var("USE_ZQ2_FROM").is_ok();
    let vtable = if overridden_zq2 {
        vec![Version {
            refspec: "use_zq2_from".to_string(),
            name: Some("use_zq2_from".to_string()),
            api_url: "http://localhost:5201".to_string(),
        }]
    } else {
        versions.versions.clone()
    };

    let index_file_path = root_path.clone().join("zq2").join("mkdocs.yaml");
    let index_file_template_path = root_path.clone().join("zq2").join("mkdocs.in.yaml");
    // Now copy the mkdocs file ..
    tokio::fs::copy(&index_file_template_path, &index_file_path).await?;
    for vrec in &vtable {
        let refspec = &vrec.refspec;
        let name: String = match vrec.name {
            None => {
                if refspec.len() > 8 {
                    refspec[..7].to_string()
                } else {
                    refspec.to_string()
                }
            }
            Some(ref val) => val.to_string(),
        };
        println!("Compiling zq2 version {name}");
        let mut cache_dir: Option<PathBuf> = Some(root_path.clone().join("cache"));
        let zq2_checkout_dir: PathBuf;
        if let Ok(val) = std::env::var("USE_ZQ2_FROM") {
            checkout = false;
            cache_dir = None;
            zq2_checkout_dir = PathBuf::from(val);
        } else if let Some(val) = &cache_dir {
            zq2_checkout_dir = val.clone().join("zq2");
        } else {
            return Err(anyhow!(
                "No zq2 specified - no cache dir and USE_ZQ2_FROM is not specified"
            ));
        }

        let id_prefix = name.to_string();
        api_prefixes.insert(format!("nav/{}", name));
        let target_dir = root_path.clone().join("zq2").join("docs").join("api");
        let target_dir_str = target_dir
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("unprintable path"))?
            .to_string();
        if checkout {
            // Check out the zq2 version
            println!("  Check out zq2 into {zq2_checkout_dir:?}");
            // Does it exist?
            // Use https so that those (me!) with yubikeys don't need to keep touching them.
            if fs::metadata(zq2_checkout_dir.clone().join(".git")).is_ok() {
                // Update.
                CommandBuilder::new()
                    .cmd("git", &["pull", "https://github.com/zilliqa/zq2", refspec])
                    .current_dir(&zq2_checkout_dir.clone())?
                    .run_logged()
                    .await?
                    .success_or("Cannot run git pull")?;
            } else if let Some(val) = &cache_dir {
                // Clone
                CommandBuilder::new()
                    .cmd("git", &["clone", "https://github.com/zilliqa/zq2"])
                    .current_dir(&val.clone())?
                    .run_logged()
                    .await?
                    .success_or("Cannot run git clone")?;
            };
            // Check out
            CommandBuilder::new()
                .cmd("git", &["checkout", refspec])
                .current_dir(&zq2_checkout_dir.clone())?
                .run_logged()
                .await?
                .success_or("Cannot run git checkout")?;
        }
        // First, zap the target
        let doc_dir = format!("{target_dir_str}/versions/{name}");
        println!(" Removing {doc_dir} ... ");
        if fs::metadata(&doc_dir).is_ok() {
            fs::remove_dir_all(&doc_dir)?;
        }

        let index_file_name = index_file_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("unprintable index file path"))?
            .to_string();
        let key_prefix = "nav".to_string();
        println!(" Generating documentation from {refspec} into {target_dir_str}...");
        let z2_dir = zq2_checkout_dir.clone();
        println!(" Running {z2_dir:?}/z2 .. ");
        // Now we can run the docgen
        CommandBuilder::new()
            .cmd(
                "scripts/z2",
                &[
                    "doc-gen",
                    &target_dir_str,
                    "--id-prefix",
                    &id_prefix,
                    "--index-file",
                    &index_file_name,
                    "--key-prefix",
                    &key_prefix,
                    "--api-url",
                    &vrec.api_url,
                ],
            )
            .current_dir(&z2_dir.clone())?
            .run_logged()
            .await?
            .success_or("Couldn't run z2")?;
    }
    // Now fix up the API paths
    fixup_api_paths(
        &index_file_path,
        &api_prefixes,
        &format!("{}/", API_DIRNAME),
    )
    .await?;
    // And reorder navigation
    reorder_top_level_navigation(&index_file_path).await?;
    Ok(())
}
