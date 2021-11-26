use std::env::consts::{ARCH, OS};
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
        let pipeline_path = dunce::canonicalize(pipeline_path)
            .with_context(|| "file or directory pipelines path were not found or inaccessible")?;

        if !pipeline_path.starts_with(&workdir_path) {
            bail!(
                "path {} can not leave current working directory {} path",
                pipeline_path.display(),
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
            // Scan directory, read all pipeline manifests and append them into the array
            let entries = std::fs::read_dir(&pipeline_path)
                .with_context(|| "error during pipeline directory reading.")?;

            // NOTE: only root level directory scanning
            'entries: for entry in entries {
                let path_resolved = dunce::canonicalize(
                    entry
                        .with_context(|| "can not get pipeline path entry.")?
                        .path(),
                )
                .with_context(|| "error during pipeline path resolving.")?;

                format!(
                    "can not get \"{}\" pipeline manifest file or inaccessible",
                    path_resolved.display()
                );

                // NOTE: we skip sub-directories for now
                if path_resolved.is_dir() {
                    continue;
                }

                let manifest = Helipad::read_manifest(&path_resolved).with_context(|| {
                    format!(
                        "can not get \"{}\" pipeline manifest file because has invalid format or inaccessible",
                        path_resolved.display()
                    )
                })?;

                if let Some(manifest) = manifest {
                    // TODO: Proper pipeline name validation
                    if manifest.name.is_empty() {
                        bail!(
                            "pipeline \"{}\" contains an invalid name.",
                            path_resolved.display()
                        );
                    }

                    // Check pipeline duplicate names
                    for man in &pipeline_manifests {
                        if man.name == manifest.name {
                            println!(
                                "Skipped: pipeline \"{}\" with name \"{}\" already exists.",
                                path_resolved.display(),
                                man.name
                            );
                            break 'entries;
                        }
                    }

                    pipeline_manifests.push(manifest);
                }
            }
        } else {
            // Or just read the pipeline manifest file and append it into the array
            let manifest = Helipad::read_manifest(&pipeline_path).with_context(|| {
                format!(
                    "can not get \"{}\" pipeline manifest file because has invalid format or inaccessible",
                    pipeline_path.display()
                )
            })?;
            if let Some(manifest) = manifest {
                pipeline_manifests.push(manifest);
            }
        }

        let os_mismatch = |name: &str, os: &str| {
            println!("WARNING: Skipping pipeline \"{}\" (os={}).", name, os);
        };
        let arch_mismatch = |name: &str, arch: &str| {
            println!("WARNING: Skipping pipeline \"{}\" (arch={}).", name, arch);
        };

        // Proper iteration over list of manifests
        // TODO: in the future we need to check for certain manifest rules like `events`.
        //  We could also parallelize pipelines based also on some rules.
        for manifest in &pipeline_manifests {
            // Check kind of pipeline
            match manifest.kind {
                // TODO: Docker pipelines
                manifest::PipelineKind::Docker => {
                    println!("TODO: Docker pipelines are not supported yet.");
                    continue;
                }

                // Host Pipelines
                manifest::PipelineKind::Host => {
                    // Check platform OS
                    match manifest.platform.os {
                        manifest::PlatformOs::Linux => {
                            if OS != "linux" {
                                os_mismatch(&manifest.name, "linux");
                                continue;
                            }
                        }
                        manifest::PlatformOs::Macos => {
                            if OS != "macos" {
                                os_mismatch(&manifest.name, "macos");
                                continue;
                            }
                        }
                        manifest::PlatformOs::Freebsd => {
                            if OS != "freebsd" {
                                os_mismatch(&manifest.name, "freebsd");
                                continue;
                            }
                        }
                        manifest::PlatformOs::Windows => {
                            if OS != "windows" {
                                os_mismatch(&manifest.name, "windows");
                                continue;
                            }
                        }
                    }

                    // Check platform Arch
                    match manifest.platform.arch {
                        manifest::PlatformArch::Amd64 => {
                            if ARCH != "x86_64" {
                                arch_mismatch(&manifest.name, "amd64");
                                continue;
                            }
                        }
                        manifest::PlatformArch::Arm64 => {
                            if ARCH != "aarch64" {
                                arch_mismatch(&manifest.name, "arm64");
                                continue;
                            }
                        }
                    }

                    pipelines::host::execute(manifest, workdir_path.as_path())?
                }
            }
        }

        Ok(())
    }

    /// Detect and read the pipeline manifest file by path.
    fn read_manifest(pipeline_file: &Path) -> Result<Option<manifest::Pipeline>> {
        // Validate TOML file extension
        let ext = pipeline_file.extension();
        if ext.is_none() || ext.unwrap().is_empty() || ext.unwrap().ne("toml") {
            return Ok(None);
        }

        // TODO: validate minimal pipeline structure (TOML file)
        let toml = manifest::read_file(pipeline_file)
            .with_context(|| "error reading pipeline toml file.")?;
        let mut unused = BTreeSet::new();
        let manifest: manifest::Pipeline = serde_ignored::deserialize(toml, |path| {
            let mut key = String::new();
            helpers::stringify(&mut key, &path);
            unused.insert(key);
        })
        .with_context(|| "error during pipeline toml file deserialization.")?;

        for key in unused {
            println!(
                "Warning: unused pipeline manifest key \"{}\" or unsuported.",
                key
            );
        }

        Ok(Some(manifest))
    }
}
