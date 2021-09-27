use crate::{Config, Context, Exec, Result, Step};
use std::collections::HashMap;

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
            .with_context(|| format!("working directory path was not found or inaccessible"))?;

        // Protect against path/directory traversal
        if !pipeline_file.starts_with(&workdir) {
            bail!("cannot leave {:?} working directory path", &workdir);
        }

        // TODO: determine if pipeline file path is a file or a directory
        if !pipeline_file.exists() {
            bail!(
                "path \"{:?}\" was not found or inaccessible",
                &pipeline_file
            );
        }

        // TODO: read pipeline manifest file
        // TODO: get commands array
        let cmds = &[];

        // let pkg_toml_path = pkg_dir.with_context(|| {
        //     let pkg_name = if is_local {
        //         pkg_dir.as_os_str().to_str().unwrap_or_default()
        //     } else {
        //         pkg_name
        //     };
        //     format!(
        //         "`paket.toml` file was not found on package `{}` or inaccessible.",
        //         pkg_name
        //     )
        // })?;

        // // Detect and read the `paket.toml` file
        // let toml = config::read_pkg_file(&pkg_toml_path)?;
        // let mut unused = BTreeSet::new();
        // let manifest: config::TomlManifest = serde_ignored::deserialize(toml, |path| {
        //     let mut key = String::new();
        //     helper_file::stringify(&mut key, &path);
        //     unused.insert(key);
        // })?;

        // for key in unused {
        //     println!(
        //         "Warning: unused Paket manifest key `{}` or unsuported.",
        //         key
        //     );
        // }

        // // Read `package` toml section
        // let toml_pkg = if manifest.package.is_some() {
        //     manifest.package.clone().unwrap()
        // } else {
        //     bail!("`paket.toml` file is empty or can not be read.")
        // };

        // TODO: append custom envs
        let mut envs: HashMap<_, _> = std::env::vars().collect();
        envs.insert("FOO".to_owned(), "bar".to_owned());

        // TODO: process steps instead of commands
        let step = Step::new(workdir, envs);
        let exc = Exec::new();

        // TODO: use a closure and pass `res` into it
        if let Some(lines) = exc.run(step, cmds)? {
            let mut res = vec![];
            let mut n = 0_usize;
            for line in lines {
                let line = line?;
                n += 1;
                println!(r#"{} {}"#, n, line);
                res.push(line);
            }

            // println!();
            // println!("JSON:");
            // println!("{}", json!(res));
        }

        Ok(())
    }
}
