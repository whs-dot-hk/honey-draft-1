pub mod hive {
    use genco::prelude::*;

    pub struct Inherit {
        pub path: String,
        pub name: String,
    }
    impl Inherit {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let nixpkgs = Inherit::new("inputs", "nixpkgs");
        ///
        /// let toks = quote! {
        ///     $nixpkgs
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) nixpkgs;",
        ///         "in",
        ///         "",
        ///         "nixpkgs",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        pub fn new(path: &str, name: &str) -> Self {
            Self {
                name: name.to_string(),
                path: path.to_string(),
            }
        }
        pub fn disko() -> Self {
            Self {
                name: String::from("disko"),
                path: String::from("inputs"),
            }
        }
    }
    impl FormatInto<Nix> for Inherit {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let nixpkgs = Inherit {
        ///     path: String::from("inputs"),
        ///     name: String::from("nixpkgs"),
        /// };
        ///
        /// let toks = quote! {
        ///     $nixpkgs
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) nixpkgs;",
        ///         "in",
        ///         "",
        ///         "nixpkgs",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            let v = nix::inherit(self.path, self.name);
            tokens.append(v)
        }
    }
    impl FormatInto<Nix> for &Inherit {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let nixpkgs = Inherit {
        ///     path: String::from("inputs"),
        ///     name: String::from("nixpkgs"),
        /// };
        ///
        /// let toks = quote! {
        ///     $(&nixpkgs)
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) nixpkgs;",
        ///         "in",
        ///         "",
        ///         "nixpkgs",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            let v = nix::inherit(self.path.clone(), self.name.clone());
            tokens.append(v)
        }
    }
    pub struct Import {
        pub inherit: Option<Inherit>,
        pub name: String,
    }
    impl Import {
        /// Create a new `Import` with inherit.
        ///
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let disko = Import::new("inputs", "disko", "nixosModules.disko");
        ///
        /// let toks = quote!($disko);
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "disko.nixosModules.disko",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        pub fn new(path: &str, var: &str, name: &str) -> Self {
            Self {
                inherit: Some(Inherit {
                    path: path.to_string(),
                    name: var.to_string(),
                }),
                name: name.to_string(),
            }
        }
        /// Create a new `Import` **without** inherit.
        ///
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let my_home_configurations = Import::new1("cell.homeConfigurations.my_home_configurations");
        ///
        /// let toks = quote!($my_home_configurations);
        ///
        /// assert_eq!(
        ///     vec![
        ///         "cell.homeConfigurations.my_home_configurations",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        pub fn new1(name: &str) -> Self {
            Self {
                inherit: None,
                name: name.to_string(),
            }
        }
        pub fn disko() -> Self {
            Self {
                inherit: Some(Inherit::disko()),
                name: String::from("nixosModules.disko"),
            }
        }
    }
    impl FormatInto<Nix> for Import {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let disko_inherit = Inherit {
        ///     name: String::from("disko"),
        ///     path: String::from("inputs"),
        /// };
        ///
        /// let disko_import = Import {
        ///     inherit: Some(disko_inherit),
        ///     name: String::from("nixosModules.disko"),
        /// };
        ///
        /// let toks = quote! {
        ///     $disko_import
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "disko.nixosModules.disko",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            if let Some(inherit) = &self.inherit {
                quote_in!(*tokens => $inherit.$(self.name))
            } else {
                quote_in!(*tokens => $(self.name))
            }
        }
    }
    impl FormatInto<Nix> for &Import {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let disko_inherit = Inherit {
        ///     name: String::from("disko"),
        ///     path: String::from("inputs"),
        /// };
        ///
        /// let disko_import = Import {
        ///     inherit: Some(disko_inherit),
        ///     name: String::from("nixosModules.disko"),
        /// };
        ///
        /// let toks = quote! {
        ///     $(&disko_import)
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "disko.nixosModules.disko",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            if let Some(inherit) = &self.inherit {
                quote_in!(*tokens => $inherit.$(self.name.clone()))
            } else {
                quote_in!(*tokens => $(self.name.clone()))
            }
        }
    }
    pub struct Imports<'a>(pub &'a Vec<Import>);
    impl FormatInto<Nix> for Imports<'_> {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let vec = vec![Import::disko()];
        ///
        /// let imports = Imports(&vec);
        ///
        /// let toks = quote! {
        ///     $imports
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "disko.nixosModules.disko",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            for import in self.0 {
                quote_in!(*tokens => $import);
                tokens.push();
            }
        }
    }
    impl FormatInto<Nix> for &Imports<'_> {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let vec = vec![Import::disko()];
        ///
        /// let imports = Imports(&vec);
        ///
        /// let toks = quote! {
        ///     $(&imports)
        /// };
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "disko.nixosModules.disko",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            for import in self.0 {
                quote_in!(*tokens => $import);
                tokens.push();
            }
        }
    }
    //pub struct Config {
    //    pub key: String,
    //    pub value: nix::Tokens,
    //}
    //impl Config {
    //    /// ```
    //    /// use genco::prelude::*;
    //    /// use honey::hive::*;
    //    ///
    //    /// let bee_system = Config::new("bee.system", quote!("x86_64-linux"));
    //    ///
    //    /// let toks = bee_system.to_tokens();
    //    ///
    //    /// assert_eq!(
    //    ///     vec![
    //    ///         "bee.system = \"x86_64-linux\"",
    //    ///     ],
    //    ///     toks.to_file_vec()?
    //    /// );
    //    ///
    //    /// let home_manager = Inherit::new("inputs", "home-manager");
    //    ///
    //    /// let bee_home = Config::new("bee.home", quote!($home_manager));
    //    ///
    //    /// let toks = bee_home.to_tokens();
    //    ///
    //    /// assert_eq!(
    //    ///     vec![
    //    ///         "let",
    //    ///         "    inherit (inputs) home-manager;",
    //    ///         "in",
    //    ///         "",
    //    ///         "bee.home = home-manager",
    //    ///     ],
    //    ///     toks.to_file_vec()?
    //    /// );
    //    ///
    //    /// # Ok::<_, genco::fmt::Error>(())
    //    /// ```
    //    pub fn new(key: &str, value: nix::Tokens) -> Self {
    //        Self {
    //            key: key.to_string(),
    //            value: value,
    //        }
    //    }
    //    pub fn to_tokens(&self) -> nix::Tokens {
    //        quote!($(&self.key) = $(&self.value))
    //    }
    //}
    pub enum ConfigurationType {
        //Config(Config),
        Import(Import),
    }
    pub struct NixosConfigurations<'a> {
        pub configurations: &'a Vec<ConfigurationType>,
        pub name: String,
    }
    impl NixosConfigurations<'_> {
        fn quote_in_imports(&self, t: &mut nix::Tokens) {
            for c in self.configurations {
                match c {
                    ConfigurationType::Import(import) => {
                        quote_in!(*t => $import);
                        t.push()
                    }
                    _ => (),
                }
            }
        }
    }
    impl FormatInto<Nix> for NixosConfigurations<'_> {
        /// ```
        /// use genco::prelude::*;
        /// use honey::hive::*;
        ///
        /// let disko = Import::disko();
        ///
        /// let my_disko_configurations = Import {
        ///     inherit: None,
        ///     name: String::from("cell.diskoConfigurations.my_disko_configurations"),
        /// };
        ///
        /// let nixos_configurations = NixosConfigurations {
        ///     configurations: &vec![
        ///         ConfigurationType::Import(disko),
        ///         ConfigurationType::Import(my_disko_configurations)
        ///     ],
        ///     name: String::from("my_nixos_configurations"),
        /// };
        ///
        /// let toks = quote!($nixos_configurations);
        ///
        /// assert_eq!(
        ///     vec![
        ///         "let",
        ///         "    inherit (inputs) disko;",
        ///         "in",
        ///         "",
        ///         "{",
        ///         "    my_nixos_configurations = {",
        ///         "        imports = [",
        ///         "            disko.nixosModules.disko",
        ///         "            cell.diskoConfigurations.my_disko_configurations",
        ///         "        ];",
        ///         "    };",
        ///         "}",
        ///     ],
        ///     toks.to_file_vec()?
        /// );
        /// # Ok::<_, genco::fmt::Error>(())
        /// ```
        fn format_into(self, tokens: &mut Tokens<Nix>) {
            quote_in! { *tokens =>
                {
                    $(&self.name) = {
                        imports = [
                            $(ref t => self.quote_in_imports(t))
                        ];
                    };
                }
            }
        }
    }
}
