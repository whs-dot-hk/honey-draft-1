use crate::hive::*;
use genco::prelude::*;

pub struct Configurations<'a> {
    pub imports: Imports<'a>,
}

impl FormatInto<Nix> for Configurations<'_> {
    /// ```
    /// use genco::prelude::*;
    /// use honey::hive::*;
    ///
    /// let vec = vec![
    ///     Import::disko(),
    ///     Import::disko_configurations("my-disko-configurations")
    /// ];
    ///
    /// let configurations = Configurations {
    ///     imports: Imports(&vec),
    /// };
    ///
    /// let toks = quote! {
    ///     $configurations
    /// };
    ///
    /// assert_eq!(
    ///     vec![
    ///         "let",
    ///         "    inherit (inputs) disko;",
    ///         "in",
    ///         "",
    ///         "{",
    ///         "    imports = [",
    ///         "        disko.nixosModules.disko",
    ///         "        cell.diskoConfigurations.my-disko-configurations",
    ///         "    ];",
    ///         "}"
    ///     ],
    ///     toks.to_file_vec()?
    /// );
    /// # Ok::<_, genco::fmt::Error>(())
    /// ```
    fn format_into(self, tokens: &mut Tokens<Nix>) {
        quote_in! { *tokens =>
            {
                imports = $(self.imports);
            }
        }
    }
}
