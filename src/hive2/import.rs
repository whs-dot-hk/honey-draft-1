use crate::hive::*;
use genco::prelude::*;

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
    /// let my_home_configurations = Import::new1("cell.homeConfigurations.my-home-configurations");
    ///
    /// let toks = quote!($my_home_configurations);
    ///
    /// assert_eq!(
    ///     vec![
    ///         "cell.homeConfigurations.my-home-configurations",
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

    pub fn nixos_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.nixosConfigurations.{}", name),
        }
    }

    pub fn nixos_profiles(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.nixosProfiles.{}", name),
        }
    }

    pub fn nixos_modules(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.nixosModules.{}", name),
        }
    }

    pub fn home_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.homeConfigurations.{}", name),
        }
    }

    pub fn home_profiles(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.homeProfiles.{}", name),
        }
    }

    pub fn home_modules(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.homeModules.{}", name),
        }
    }

    pub fn hardware_profiles(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.hardwareProfiles.{}", name),
        }
    }

    pub fn disko_configurations(name: &str) -> Self {
        Self {
            inherit: None,
            name: format!("cell.diskoConfigurations.{}", name),
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
    /// let vec = vec![
    ///     Import::hardware_profiles("my-hardware-profile"),
    ///     Import::nixos_profiles("my-nix-profile"),
    /// ];
    ///
    /// let imports = Imports(&vec);
    ///
    /// let toks = quote! {
    ///     $imports
    /// };
    ///
    /// assert_eq!(
    ///     vec![
    ///         "[",
    ///         "    cell.hardwareProfiles.my-hardware-profile",
    ///         "    cell.nixosProfiles.my-nix-profile",
    ///         "]"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        tokens.append("[");
        tokens.indent();
        for import in self.0 {
            quote_in!(*tokens => $import);
            tokens.push();
        }
        tokens.unindent();
        tokens.append("]");
    }
}
impl FormatInto<Nix> for &Imports<'_> {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let vec = vec![
    ///     Import::disko(),
    ///     Import::disko_configurations("my-disko-configurations")
    /// ];
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
    ///         "[",
    ///         "    disko.nixosModules.disko",
    ///         "    cell.diskoConfigurations.my-disko-configurations",
    ///         "]",
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        tokens.append("[");
        tokens.indent();
        for import in self.0 {
            quote_in!(*tokens => $import);
            tokens.push();
        }
        tokens.unindent();
        tokens.append("]");
    }
}
