use std::{collections::BTreeSet, path::Path, vec};

use crate::{helpers, manifest, pipelines, Config, Context, Result};

pub struct Helipad {
    opts: Config,
}

impl Helipad {
    pub fn new(opts: Config) -> Self {
        Self { opts }
    }

    // Read all pipeline manifests available and executes evey step.
    pub fn start(&self) -> Result {
        let pipeline_path = &self.opts.config;
        let workdir_path = std::env::current_dir()
            .with_context(|| "working directory path was not found or inaccessible")?;

        // Protect against path/directory traversal
        let pipeline_path = pipeline_path
            .canonicalize()
            .with_context(|| "file or directory pipelines path were not found or inaccessible")?;

        if !pipeline_path.starts_with(&workdir_path) {
            bail!(
                "path {} can not leave current working directory path",
                workdir_path.display()
            );
        }

        // Determine if pipeline file path is a file or a directory
        let pipeline_meta = std::fs::metadata(&pipeline_path)
            .with_context(|| "can not get pipeline file or dir metadata.")?;

        let file_type = pipeline_meta.file_type();
        let mut is_dir = false;

        if file_type.is_dir() {
            is_dir = true
        } else if file_type.is_file() {
            is_dir = false
        } else if file_type.is_symlink() {
            let symlink_meta = std::fs::symlink_metadata(&pipeline_path)?;
            if symlink_meta.is_dir() {
                is_dir = true
            }
        } else {
            bail!(
                "unknown path or directory for \"{}\"",
                pipeline_path.display()
            );
        }

        let mut pipeline_manifests: Vec<manifest::Pipeline> = vec![];

        if is_dir {
            // Scan directory, read all pipeline manifests and put them in an the array
            let entries = std::fs::read_dir(&pipeline_path)
                .with_context(|| "error during pipeline directory reading.")?;

            // NOTE: only root level directory scanning
            for entry in entries {
                let path_resolved = entry
                    .with_context(|| "can not get pipeline path entry.")?
                    .path()
                    .canonicalize()
                    .with_context(|| "error during pipeline path resolving.")?;

                format!(
                    "can not get \"{}\" pipeline manifest file or inaccessible",
                    path_resolved.display()
                );

                // NOTE: we skip sub-directories for now
                if path_resolved.is_dir() {
                    continue;
                }

                let manifest = Helipad::get_manifest(&path_resolved).with_context(|| {
                    format!(
                        "can not get \"{}\" pipeline manifest file or inaccessible",
                        pipeline_path.display()
                    )
                })?;
                pipeline_manifests.push(manifest);
            }
        } else {
            // Or just read the pipeline manifest file and put it in an the array
            let manifest = Helipad::get_manifest(&pipeline_path).with_context(|| {
                format!(
                    "can not get \"{}\" pipeline manifest file or inaccessible",
                    pipeline_path.display()
                )
            })?;
            pipeline_manifests.push(manifest);
        }

        // Proper iteration over list of manifests
        // TODO: in the future we need to check for certain manifest rules like `events`.
        // We could also parallelize pipelines based also en some rules.
        for manifest in &pipeline_manifests {
            match manifest.kind {
                manifest::PipelineKind::Docker => {
                    // TODO: Docker is not supported yet
                    println!("TODO: Docker is not supported yet")
                }
                manifest::PipelineKind::Host => {
                    pipelines::host::run(manifest, workdir_path.as_path())?
                }
            }
        }

        Ok(())
    }

    /// Detect and read the pipeline manifest file by path.
    fn get_manifest(pipeline_file: &Path) -> Result<manifest::Pipeline> {
        let toml = manifest::read_file(pipeline_file)?;
        let mut unused = BTreeSet::new();
        let manifest: manifest::Pipeline = serde_ignored::deserialize(toml, |path| {
            let mut key = String::new();
            helpers::stringify(&mut key, &path);
            unused.insert(key);
        })
        .with_context(|| "error during pipeline file toml deserialize.")?;

        for key in unused {
            println!(
                "Warning: unused pipeline manifest key \"{}\" or unsuported.",
                key
            );
        }

        Ok(manifest)
    }
}
