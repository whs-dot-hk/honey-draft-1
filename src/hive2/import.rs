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
