/// `Flag` type.
///
/// Option flag struct
#[derive(Clone)]
pub struct Flag {
    /// Flag name
    pub name: String,
    /// Flag usage
    pub usage: String,
    /// Flag type
    pub flag_type: FlagType,
    /// Flag alias
    pub alias: Option<Vec<String>>,
}

/// `FlagType` enum
#[derive(PartialOrd, PartialEq, Clone)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

/// `FlagValue` enum
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(isize),
    Float(f64),
}

impl Flag {
    /// Create new instance of `Flag`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Flag, FlagType};
    ///
    /// let bool_flag = Flag::new("bool", "cli cmd [arg] --bool", FlagType::Bool);
    /// let float_flag = Flag::new("float", "cli cmd [arg] --float [float]", FlagType::Float);
    /// ```
    pub fn new<T: Into<String>>(name: T, usage: T, flag_type: FlagType) -> Self {
        let name = name.into();
        if name.starts_with('-') {
            panic!(format!(
                r#""{}" is invalid flag name. Flag name cannnot start with "-"."#,
                name
            ))
        }
        if name.contains('=') {
            panic!(format!(
                r#""{}" is invalid flag name. Flag name cannnot contain "="."#,
                name
            ))
        }
        if name.contains(' ') {
            panic!(format!(
                r#""{}" is invalid flag name. Flag name cannnot contain whitespaces."#,
                name
            ))
        }

        Self {
            name,
            usage: usage.into(),
            flag_type,
            alias: None,
        }
    }

    /// Set alias of the flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Flag, FlagType};
    ///
    /// let bool_flag = Flag::new("bool", "cli cmd [arg] --bool", FlagType::Bool)
    ///     .alias("b");
    ///
    /// let string_flag = Flag::new("string", "cli cmd [arg] --string [string]", FlagType::String)
    ///     .alias("s")
    ///     .alias("str");
    /// ```
    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into());
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

    /// Get flag position from command line argument
    pub fn option_index(&self, v: &[String]) -> Option<usize> {
        match &self.alias {
            Some(alias) => v.iter().position(|r| {
                r == &format!("--{}", &self.name) || alias.iter().any(|a| r == &format!("-{}", a))
            }),
            None => v.iter().position(|r| r == &format!("--{}", &self.name)),
        }
    }

    /// Get flag value
    pub fn value(&self, v: &[String]) -> Option<FlagValue> {
        match self.flag_type {
            FlagType::Bool => match &self.alias {
                Some(alias) => Some(FlagValue::Bool(
                    v.contains(&format!("--{}", self.name))
                        || alias.iter().any(|a| v.contains(&format!("-{}", a))),
                )),
                None => Some(FlagValue::Bool(v.contains(&format!("--{}", self.name)))),
            },
            FlagType::String => match self.option_index(&v) {
                Some(index) => Some(FlagValue::String(v[index + 1].to_owned())),
                None => None,
            },
            FlagType::Int => match self.option_index(&v) {
                Some(index) => Some(FlagValue::Int(
                    v[index + 1].parse::<isize>().unwrap_or_else(|_| {
                        panic!(format!("The value of `{}` flag should be int.", self.name))
                    }),
                )),
                None => None,
            },
            FlagType::Float => match self.option_index(&v) {
                Some(index) => Some(FlagValue::Float(
                    v[index + 1].parse::<f64>().unwrap_or_else(|_| {
                        panic!(format!(
                            "The value of `{}` flag should be float.",
                            self.name
                        ))
                    }),
                )),
                None => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Flag, FlagType, FlagValue};

    #[test]
    fn opiton_index() {
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "-a".to_string(),
            "--bool".to_string(),
            "-c".to_string(),
        ];
        {
            let f = Flag::new("bool", "", FlagType::Bool);
            assert_eq!(f.option_index(&v), Some(3));
        }
        {
            let f = Flag::new("age", "", FlagType::Bool).alias("a");
            assert_eq!(f.option_index(&v), Some(2));
        }
        {
            let f = Flag::new("dance", "", FlagType::Bool);
            assert_eq!(f.option_index(&v), None);
        }
    }

    #[test]
    #[should_panic]
    fn construct_fail_1() {
        Flag::new("bo=ol", "", FlagType::Bool);
    }

    #[test]
    #[should_panic]
    fn construct_fail_2() {
        Flag::new("------bool", "", FlagType::Bool);
    }

    #[test]
    #[should_panic]
    fn construct_fail_3() {
        Flag::new("cool flag", "", FlagType::Bool);
    }

    #[test]
    fn bool_flag_test() {
        let bool_flag = Flag::new("bool", "", FlagType::Bool);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
        ];

        match bool_flag.value(&v) {
            Some(FlagValue::Bool(val)) => assert!(val),
            _ => assert!(false),
        }
    }

    #[test]
    fn string_flag_test() {
        let string_flag = Flag::new("string", "", FlagType::String);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--string".to_string(),
            "test".to_string(),
        ];

        match string_flag.value(&v) {
            Some(FlagValue::String(val)) => assert_eq!("test".to_string(), val),
            _ => assert!(false),
        }
    }

    #[test]
    fn int_flag_test() {
        let int_flag = Flag::new("int", "", FlagType::Int);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--int".to_string(),
            "100".to_string(),
        ];

        match int_flag.value(&v) {
            Some(FlagValue::Int(val)) => assert_eq!(100, val),
            _ => assert!(false),
        }
    }

    #[test]
    fn float_flag_test() {
        let float_flag = Flag::new("float", "", FlagType::Float);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ];

        match float_flag.value(&v) {
            Some(FlagValue::Float(val)) => assert_eq!(1.23, val),
            _ => assert!(false),
        }
    }
}
