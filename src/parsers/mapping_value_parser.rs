use crate::mapping::{Mapping, MappingType};

use clap::{
    builder::StringValueParser,
    error::{ContextKind, ContextValue, ErrorKind},
    Error,
};

#[derive(Clone, Copy)]
pub struct MappingValueParser;
impl clap::builder::TypedValueParser for MappingValueParser {
    type Value = Mapping;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let error = || {
            let mut err = Error::new(ErrorKind::ValueValidation).with_cmd(cmd);
            if let Some(arg) = arg {
                err.insert(
                    ContextKind::InvalidArg,
                    ContextValue::String(arg.to_string()),
                );
            }
            err
        };

        println!("{arg:?}");

        let mapping_type = match arg {
            Some(arg) => match arg.get_id().as_str() {
                "uid" => Ok(MappingType::Uid),
                "gid" => Ok(MappingType::Gid),
                arg_id => {
                    let mut err = error();
                    err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String(arg_id.to_string()),
                    );
                    Err(err)
                }
            },
            None => Ok(MappingType::Both),
        }?;

        let value = StringValueParser::new().parse_ref(cmd, arg, value)?;
        Mapping::from_str_with_type(&value, mapping_type).map_err(|mapping_err| {
            let mut err = error();
            err.insert(
                ContextKind::Custom,
                ContextValue::String(mapping_err.message),
            );
            err
        })
    }
}

impl clap::builder::ValueParserFactory for Mapping {
    type Parser = MappingValueParser;

    fn value_parser() -> Self::Parser {
        MappingValueParser
    }
}
