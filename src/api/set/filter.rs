use itertools::Itertools;
use std::fmt::Display;

const SEP_AND: &str = ",";
const SEP_OR: &str = "|";

#[derive(Clone, Debug)]
pub struct SetFilterBuilder {
    filter: String,
}

impl SetFilterBuilder {
    fn new() -> SetFilterBuilder {
        SetFilterBuilder {
            filter: String::new(),
        }
    }

    /// Creates a Setilter with the specified filter parameters
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder
    ///     .name("Khans of Tarkir")
    ///     .block("Khans of Tarkir")
    ///     .build();
    /// assert!(filter == SetFilter("name=Khans of Tarkir&block=Khans of Tarkir".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn build(self) -> SetFilter {
        SetFilter(self.filter)
    }

    /// Create a custom filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.custom("name", "Khans of Tarkir")
    ///     .build();
    /// assert!(filter == SetFilter("name=Khans of Tarkir".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn custom<'a, T>(mut self, key: T, value: T) -> SetFilterBuilder
        where
            T: Into<&'a str>,
    {
        self.add_filter(key.into(), value.into());
        self
    }

    /// Every set that (partially) matches the specified name will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.name("Shock")
    ///     .build();
    /// assert!(filter == SetFilter("name=Shock".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn name<'a, T>(mut self, name: T) -> SetFilterBuilder
        where
            T: Into<&'a str>,
    {
        self.add_filter("name", name.into());
        self
    }

    /// Every set that (partially) matches the specified block will match the filter
    ///
    /// ```
    /// # use mtgio_client::prelude::*;
    /// let builder = SetFilter::builder();
    /// let filter = builder.block("Shock")
    ///     .build();
    /// assert!(filter == SetFilter("block=Shock".to_string()))
    /// ```
    #[allow(dead_code)]
    pub fn block<'a, T>(mut self, block: T) -> SetFilterBuilder
        where
            T: Into<&'a str>,
    {
        self.add_filter("block", block.into());
        self
    }

    fn add_filter<T>(&mut self, key: T, values: T)
        where
            T: Display,
    {
        if !self.filter.is_empty() {
            self.filter = [&self.filter, "&"].join("");
        }
        self.filter = self.filter.clone() + &[key, values].into_iter().join("=")
    }
}

/// Wrapper around the filter string to be used for filtered set api requests
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SetFilter(pub String);

impl SetFilter {
    /// Creates a new SetFilterBuilder
    #[allow(dead_code)]
    pub fn builder() -> SetFilterBuilder {
        SetFilterBuilder::new()
    }
}