use std::{collections::BTreeSet, vec};

use crate::{helpers, manifest, pipelines, Config, Context, Result};

pub struct Helipad {
    opts: Config,
}

impl Helipad {
    pub fn new(opts: Config) -> Self {
        Self { opts }
    }

    // TODO: execute steps instead of commands
    pub fn start(&self) -> Result {
        let pipeline_file = &self.opts.config;
        let workdir = std::env::current_dir()
            .with_context(|| "working directory path was not found or inaccessible")?;

        // Protect against path/directory traversal
        let pipeline_file = pipeline_file
            .canonicalize()
            .with_context(|| "file or directory path was not found or inaccessible")?;

        if !pipeline_file.starts_with(&workdir) {
            bail!("cannot leave {:?} current working directory path", &workdir);
        }

        // Determine if pipeline file path is a file or a directory
        let pipeline_meta = std::fs::metadata(&pipeline_file)?;
        let file_type = pipeline_meta.file_type();
        let mut is_dir = false;

        if file_type.is_dir() {
            is_dir = true
        } else if file_type.is_file() {
            is_dir = false
        } else if file_type.is_symlink() {
            let symlink_meta = std::fs::symlink_metadata(&pipeline_file)?;
            if symlink_meta.is_dir() {
                is_dir = true
            }
        } else {
            bail!("unknown path or directory for \"{:?}\"", &pipeline_file);
        }

        let mut pipeline_manifests: Vec<manifest::Pipeline> = vec![];

        if is_dir {
            // TODO: scan dir, read all piepelines and put them in an array
        } else {
            // TODO: Detect and read the pipeline manifest file
            let toml = manifest::read_file(&pipeline_file)?;
            let mut unused = BTreeSet::new();
            let manifest: manifest::Pipeline = serde_ignored::deserialize(toml, |path| {
                let mut key = String::new();
                helpers::stringify(&mut key, &path);
                unused.insert(key);
            })?;

            for key in unused {
                println!(
                    "Warning: unused pipeline manifest key `{}` or unsuported.",
                    key
                );
            }

            pipeline_manifests.push(manifest);
        }

        // TODO: for testing purposes we use just one entry
        // use proper iteration for the list of manifests
        let manifest = &pipeline_manifests[0];

        match manifest.kind {
            manifest::PipelineKind::Docker => {
                // TODO: Docker not supported yet
                println!("TODO: Docker")
            }
            manifest::PipelineKind::Host => pipelines::host::run(manifest, workdir)?,
        }

        Ok(())
    }
}
