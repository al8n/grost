#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
/// Abstract Syntax Tree (AST) type definitions for the Grost DSL.
///
/// This module contains all the AST node types that represent the semantic
/// structure of parsed Grost DSL documents. These types are produced by
/// converting the Concrete Syntax Tree (CST) from the pest parser.
pub mod ast {}
/// Parser implementation for the Grost DSL using the [`pest`].
///
/// This module provides the parser that converts Grost DSL source text into
/// a Concrete Syntax Tree (CST), which can then be transformed into the AST
/// types defined in the [`ast`] module.
pub mod parser {
    /// A parser for the Grost DSL
    #[grammar = "../grost.pest"]
    pub struct GrostParser;
    #[allow(non_upper_case_globals)]
    const _PEST_GRAMMAR_GrostParser: [&'static str; 1usize] = [
        "// Enhanced GraphQL Grammar with Generics, Sets, and Maps\n// \n// Based on the original GraphQL grammar from async-graphql project:\n// https://github.com/async-graphql/async-graphql\n// \n// Original Copyright: async-graphql contributors\n// Original License: Apache-2.0 OR MIT\n// \n// Enhancements include:\n// - Generic type parameters and where clauses\n// - Set types: <T> for sets\n// - Map types: <K, V> for maps\n// \n// This enhanced version maintains the same dual license as the original:\n// Apache-2.0 OR MIT\n\nWHITESPACE = _{ \" \" | \",\" | \"\\t\" | \"\\u{feff}\" | line_terminator }\nCOMMENT = _{ \"#\" ~ (!line_terminator ~ ANY)* }\nline_terminator = @{ \"\\r\\n\" | \"\\r\" | \"\\n\" }\n\n// Executable //\n\nexecutable_document   = { SOI ~ executable_definition+ ~ EOI }\nexecutable_definition = { operation_definition | fragment_definition }\n\noperation_definition       = { named_operation_definition | selection_set }\nnamed_operation_definition = { operation_type ~ name? ~ variable_definitions? ~ directives? ~ selection_set }\nvariable_definitions       = { \"(\" ~ variable_definition* ~ \")\" }\nvariable_definition        = { variable ~ \":\" ~ type_ ~ directives? ~ default_value? }\n\nselection_set = { \"{\" ~ selection+ ~ \"}\" }\nselection = { field | inline_fragment | fragment_spread }\nfield = { alias? ~ name ~ arguments? ~ directives? ~ selection_set? }\nalias = { name ~ \":\" }\nfragment_spread = { \"...\" ~ !type_condition ~ name ~ directives? }\ninline_fragment = { \"...\" ~ type_condition? ~ directives? ~ selection_set }\n\nfragment_definition = { \"fragment\" ~ name ~ type_condition ~ directives? ~ selection_set }\ntype_condition = ${ \"on\" ~ WHITESPACE+ ~ name }\n\n// Service //\n\nservice_document       = { SOI ~ type_system_definition+ ~ EOI }\ntype_system_definition = { schema_definition | type_definition | directive_definition }\n\nschema_definition = {\n\t\"schema\" ~ const_directives? ~ \"{\" ~ operation_type_definition+ ~ \"}\"\n\t| extend ~ \"schema\" ~ (const_directives? ~ \"{\" ~ operation_type_definition+ ~ \"}\" | const_directives)\n}\noperation_type_definition = { operation_type ~ \":\" ~ name }\n\ntype_definition = { scalar_type | object_type | interface_type | union_type | enum_type | input_object_type }\n\nscalar_type = {\n\tstring? ~ \"scalar\" ~ name ~ generic_type_parameters? ~ where_clause? ~ const_directives?\n\t| extend ~ \"scalar\" ~ name ~ generic_type_parameters? ~ where_clause? ~ const_directives\n}\n\nobject_type = {\n\tstring? ~ \"type\" ~ name ~ generic_type_parameters? ~ implements_interfaces? ~ where_clause? ~ const_directives? ~ fields_definition?\n\t| extend ~ \"type\" ~ name ~ generic_type_parameters? ~ (implements_interfaces? ~ where_clause? ~ (const_directives? ~ fields_definition | const_directives) | implements_interfaces)\n}\nimplements_interfaces = { \"implements\" ~ \"&\"? ~ generic_type ~ (\"&\" ~ generic_type)* }\n\ninterface_type = {\n\tstring? ~ \"interface\" ~ name ~ generic_type_parameters? ~ implements_interfaces? ~ where_clause? ~ const_directives? ~ fields_definition?\n\t| extend ~ \"interface\" ~ name ~ generic_type_parameters? ~ implements_interfaces? ~ where_clause? ~ (const_directives? ~ fields_definition | const_directives)\n}\n\nfields_definition = { \"{\" ~ field_definition+ ~ \"}\" }\nfield_definition = { string? ~ name ~ arguments_definition? ~ \":\" ~ type_ ~ const_directives? }\n\nunion_type = {\n\tstring? ~ \"union\" ~ name ~ generic_type_parameters? ~ where_clause? ~ const_directives? ~ union_member_types?\n\t| extend ~ \"union\" ~ name ~ generic_type_parameters? ~ where_clause? ~ (const_directives? ~ union_member_types | const_directives)\n}\nunion_member_types = { \"=\" ~ \"|\"? ~ generic_type ~ (\"|\" ~ generic_type)* }\n\nenum_type = {\n\tstring? ~ \"enum\" ~ name ~ const_directives? ~ enum_values?\n\t| extend ~ \"enum\" ~ name ~ (const_directives? ~ enum_values | const_directives)\n}\nenum_values = { \"{\" ~ enum_value_definition+ ~ \"}\" }\nenum_value_definition = { string? ~ enum_value ~ const_directives? }\n\ninput_object_type = {\n\tstring? ~ \"input\" ~ name ~ generic_type_parameters? ~ where_clause? ~ const_directives? ~ input_fields_definition?\n\t| extend ~ \"input\" ~ name ~ generic_type_parameters? ~ where_clause? ~ (const_directives? ~ input_fields_definition | const_directives)\n}\ninput_fields_definition = { \"{\" ~ input_value_definition+ ~ \"}\" }\n\nextend = { \"extend\" }\n\ndirective_definition = { string? ~ \"directive\" ~ \"@\" ~ name ~ arguments_definition? ~ repeatable ~ \"on\" ~ directive_locations }\nrepeatable = { \"repeatable\"? }\ndirective_locations = { \"|\"? ~ directive_location ~ (\"|\" ~ directive_location)* }\ndirective_location = {\n\t\"QUERY\"\n\t| \"MUTATION\"\n\t| \"SUBSCRIPTION\"\n\t| \"FIELD_DEFINITION\"\n\t| \"FIELD\"\n\t| \"FRAGMENT_DEFINITION\"\n\t| \"FRAGMENT_SPREAD\"\n\t| \"INLINE_FRAGMENT\"\n\t| \"VARIABLE_DEFINITION\"\n\t| \"SCHEMA\"\n\t| \"SCALAR\"\n\t| \"OBJECT\"\n\t| \"ARGUMENT_DEFINITION\"\n\t| \"INTERFACE\"\n\t| \"UNION\"\n\t| \"ENUM_VALUE\"\n\t| \"ENUM\"\n\t| \"INPUT_OBJECT\"\n\t| \"INPUT_FIELD_DEFINITION\"\n}\n\narguments_definition = { \"(\" ~ input_value_definition+ ~ \")\" }\n\ninput_value_definition = { string? ~ name ~ \":\" ~ type_ ~ default_value? ~ const_directives? }\n\n// Generics Support //\n\ngeneric_type_parameters = { \"<\" ~ generic_type_parameter ~ (\",\" ~ generic_type_parameter)* ~ \">\" }\ngeneric_type_parameter = { name }\n\ngeneric_type_arguments = { \"<\" ~ type_ ~ (\",\" ~ type_)* ~ \">\" }\n\ngeneric_type = { name ~ generic_type_arguments? }\n\n// Where clause for generic constraints\nwhere_clause = { \"where\" ~ where_constraint ~ (\",\" ~ where_constraint)* }\nwhere_constraint = { \n    name ~ \":\" ~ constraint_bound ~ (\"+\" ~ constraint_bound)*\n}\nconstraint_bound = { name ~ generic_type_arguments? }\n\n// Common //\n\noperation_type = { \"query\" | \"mutation\" | \"subscription\" }\n\ndefault_value = { \"=\" ~ const_value }\n\n// Enhanced type system with generics, sets, and maps\ntype_ = @{ type_base ~ \"!\"? }\ntype_base = {\n\tlist_type\n\t| set_type\n\t| map_type\n\t| generic_type\n}\n\nlist_type = { \"[\" ~ type_ ~ \"]\" }\nset_type = { \"<\" ~ type_ ~ \">\" }\nmap_type = { \"<\" ~ type_ ~ \",\" ~ type_ ~ \">\" }\n\nconst_value = {            number | string | boolean | null | enum_value | const_list | const_set | const_map | const_object }\nvalue       = { variable | number | string | boolean | null | enum_value |       list |       set |       map |       object }\n\nvariable = { \"$\" ~ name }\n\nnumber     = @{ (float | int) ~ !name_start }\nfloat      = { int ~ ((fractional ~ exponent) | fractional | exponent) }\nfractional = { \".\" ~ ASCII_DIGIT+ }\nexponent   = { (\"E\" | \"e\") ~ (\"+\" | \"-\")? ~ ASCII_DIGIT+ }\nint        = { \"-\"? ~ (\"0\" | (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)) }\n\nstring = ${ (\"\\\"\\\"\\\"\" ~ block_string_content ~ \"\\\"\\\"\\\"\") | (\"\\\"\" ~ string_content ~ \"\\\"\") }\nblock_string_content = @{ block_string_character* }\nblock_string_character = {\n\t(!(\"\\\"\\\"\\\"\" | \"\\\\\\\"\\\"\\\"\") ~ ANY)\n\t| \"\\\\\\\"\\\"\\\"\"\n}\nstring_content = @{ string_character* }\nstring_character = {\n\t(!(\"\\\"\" | \"\\\\\" | line_terminator) ~ ANY)\n\t| (\"\\\\\" ~ (\"\\\"\" | \"\\\\\" | \"/\" | \"b\" | \"f\" | \"n\" | \"r\" | \"t\"))\n\t| (\"\\\\u\" ~ unicode_scalar_value_hex)\n}\n// Spec inconsistency:\n// In GraphQL, strings can contain any Unicode code point. However in Rust strings can only contain\n// Unicode Scalar Values. To avoid having to use Vec<u8> everywhere we deviate from the spec\n// slightly and disallow non-scalar value code points at the parsing level.\nunicode_scalar_value_hex = { !(^\"d\" ~ (\'8\'..\'9\' | \'a\'..\'f\' | \'A\'..\'F\')) ~ ASCII_HEX_DIGIT{4} }\n\nboolean = { \"true\" | \"false\" }\n\nnull = { \"null\" }\n\nenum_value = ${ !(boolean | null) ~ name }\n\nconst_list = { \"[\" ~ const_value* ~ \"]\" }\nlist       = { \"[\" ~       value* ~ \"]\" }\n\nconst_set = { \"<\" ~ const_value* ~ \">\" }\nset       = { \"<\" ~       value* ~ \">\" }\n\nconst_map = { \"<\" ~ const_map_entry* ~ \">\" }\nmap       = { \"<\" ~       map_entry* ~ \">\" }\nconst_map_entry = { const_value ~ \":\" ~ const_value }\nmap_entry       = {       value ~ \":\" ~       value }\n\nconst_object = { \"{\" ~ const_object_field* ~ \"}\" }\nobject       = { \"{\" ~       object_field* ~ \"}\" }\nconst_object_field = { name ~ \":\" ~ const_value }\nobject_field       = { name ~ \":\" ~       value }\n\nconst_directives = { const_directive+ }\ndirectives       = {       directive+ }\nconst_directive  = { \"@\" ~ name ~ const_arguments? }\ndirective        = { \"@\" ~ name ~       arguments? }\n\nconst_arguments = { \"(\" ~ const_argument+ ~ \")\" }\narguments       = { \"(\" ~       argument+ ~ \")\" }\nconst_argument = { name ~ \":\" ~ const_value }\nargument       = { name ~ \":\" ~       value }\n\nname_start = @{ (ASCII_ALPHA | \"_\") }\nname = @{ name_start ~ (ASCII_ALPHA | ASCII_DIGIT | \"_\")* }",
    ];
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    pub enum Rule {
        ///End-of-input
        EOI,
        WHITESPACE,
        COMMENT,
        line_terminator,
        executable_document,
        executable_definition,
        operation_definition,
        named_operation_definition,
        variable_definitions,
        variable_definition,
        selection_set,
        selection,
        field,
        alias,
        fragment_spread,
        inline_fragment,
        fragment_definition,
        type_condition,
        service_document,
        type_system_definition,
        schema_definition,
        operation_type_definition,
        type_definition,
        scalar_type,
        object_type,
        implements_interfaces,
        interface_type,
        fields_definition,
        field_definition,
        union_type,
        union_member_types,
        enum_type,
        enum_values,
        enum_value_definition,
        input_object_type,
        input_fields_definition,
        extend,
        directive_definition,
        repeatable,
        directive_locations,
        directive_location,
        arguments_definition,
        input_value_definition,
        generic_type_parameters,
        generic_type_parameter,
        generic_type_arguments,
        generic_type,
        where_clause,
        where_constraint,
        constraint_bound,
        operation_type,
        default_value,
        type_,
        type_base,
        list_type,
        set_type,
        map_type,
        const_value,
        value,
        variable,
        number,
        float,
        fractional,
        exponent,
        int,
        string,
        block_string_content,
        block_string_character,
        string_content,
        string_character,
        unicode_scalar_value_hex,
        boolean,
        null,
        enum_value,
        const_list,
        list,
        const_set,
        set,
        const_map,
        map,
        const_map_entry,
        map_entry,
        const_object,
        object,
        const_object_field,
        object_field,
        const_directives,
        directives,
        const_directive,
        directive,
        const_arguments,
        arguments,
        const_argument,
        argument,
        name_start,
        name,
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::clone::Clone for Rule {
        #[inline]
        fn clone(&self) -> Rule {
            *self
        }
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::marker::Copy for Rule {}
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::fmt::Debug for Rule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Rule::EOI => "EOI",
                    Rule::WHITESPACE => "WHITESPACE",
                    Rule::COMMENT => "COMMENT",
                    Rule::line_terminator => "line_terminator",
                    Rule::executable_document => "executable_document",
                    Rule::executable_definition => "executable_definition",
                    Rule::operation_definition => "operation_definition",
                    Rule::named_operation_definition => "named_operation_definition",
                    Rule::variable_definitions => "variable_definitions",
                    Rule::variable_definition => "variable_definition",
                    Rule::selection_set => "selection_set",
                    Rule::selection => "selection",
                    Rule::field => "field",
                    Rule::alias => "alias",
                    Rule::fragment_spread => "fragment_spread",
                    Rule::inline_fragment => "inline_fragment",
                    Rule::fragment_definition => "fragment_definition",
                    Rule::type_condition => "type_condition",
                    Rule::service_document => "service_document",
                    Rule::type_system_definition => "type_system_definition",
                    Rule::schema_definition => "schema_definition",
                    Rule::operation_type_definition => "operation_type_definition",
                    Rule::type_definition => "type_definition",
                    Rule::scalar_type => "scalar_type",
                    Rule::object_type => "object_type",
                    Rule::implements_interfaces => "implements_interfaces",
                    Rule::interface_type => "interface_type",
                    Rule::fields_definition => "fields_definition",
                    Rule::field_definition => "field_definition",
                    Rule::union_type => "union_type",
                    Rule::union_member_types => "union_member_types",
                    Rule::enum_type => "enum_type",
                    Rule::enum_values => "enum_values",
                    Rule::enum_value_definition => "enum_value_definition",
                    Rule::input_object_type => "input_object_type",
                    Rule::input_fields_definition => "input_fields_definition",
                    Rule::extend => "extend",
                    Rule::directive_definition => "directive_definition",
                    Rule::repeatable => "repeatable",
                    Rule::directive_locations => "directive_locations",
                    Rule::directive_location => "directive_location",
                    Rule::arguments_definition => "arguments_definition",
                    Rule::input_value_definition => "input_value_definition",
                    Rule::generic_type_parameters => "generic_type_parameters",
                    Rule::generic_type_parameter => "generic_type_parameter",
                    Rule::generic_type_arguments => "generic_type_arguments",
                    Rule::generic_type => "generic_type",
                    Rule::where_clause => "where_clause",
                    Rule::where_constraint => "where_constraint",
                    Rule::constraint_bound => "constraint_bound",
                    Rule::operation_type => "operation_type",
                    Rule::default_value => "default_value",
                    Rule::type_ => "type_",
                    Rule::type_base => "type_base",
                    Rule::list_type => "list_type",
                    Rule::set_type => "set_type",
                    Rule::map_type => "map_type",
                    Rule::const_value => "const_value",
                    Rule::value => "value",
                    Rule::variable => "variable",
                    Rule::number => "number",
                    Rule::float => "float",
                    Rule::fractional => "fractional",
                    Rule::exponent => "exponent",
                    Rule::int => "int",
                    Rule::string => "string",
                    Rule::block_string_content => "block_string_content",
                    Rule::block_string_character => "block_string_character",
                    Rule::string_content => "string_content",
                    Rule::string_character => "string_character",
                    Rule::unicode_scalar_value_hex => "unicode_scalar_value_hex",
                    Rule::boolean => "boolean",
                    Rule::null => "null",
                    Rule::enum_value => "enum_value",
                    Rule::const_list => "const_list",
                    Rule::list => "list",
                    Rule::const_set => "const_set",
                    Rule::set => "set",
                    Rule::const_map => "const_map",
                    Rule::map => "map",
                    Rule::const_map_entry => "const_map_entry",
                    Rule::map_entry => "map_entry",
                    Rule::const_object => "const_object",
                    Rule::object => "object",
                    Rule::const_object_field => "const_object_field",
                    Rule::object_field => "object_field",
                    Rule::const_directives => "const_directives",
                    Rule::directives => "directives",
                    Rule::const_directive => "const_directive",
                    Rule::directive => "directive",
                    Rule::const_arguments => "const_arguments",
                    Rule::arguments => "arguments",
                    Rule::const_argument => "const_argument",
                    Rule::argument => "argument",
                    Rule::name_start => "name_start",
                    Rule::name => "name",
                },
            )
        }
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::cmp::Eq for Rule {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::hash::Hash for Rule {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::cmp::Ord for Rule {
        #[inline]
        fn cmp(&self, other: &Rule) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::marker::StructuralPartialEq for Rule {}
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::cmp::PartialEq for Rule {
        #[inline]
        fn eq(&self, other: &Rule) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    #[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
    impl ::core::cmp::PartialOrd for Rule {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Rule,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl Rule {
        pub fn all_rules() -> &'static [Rule] {
            &[
                Rule::WHITESPACE,
                Rule::COMMENT,
                Rule::line_terminator,
                Rule::executable_document,
                Rule::executable_definition,
                Rule::operation_definition,
                Rule::named_operation_definition,
                Rule::variable_definitions,
                Rule::variable_definition,
                Rule::selection_set,
                Rule::selection,
                Rule::field,
                Rule::alias,
                Rule::fragment_spread,
                Rule::inline_fragment,
                Rule::fragment_definition,
                Rule::type_condition,
                Rule::service_document,
                Rule::type_system_definition,
                Rule::schema_definition,
                Rule::operation_type_definition,
                Rule::type_definition,
                Rule::scalar_type,
                Rule::object_type,
                Rule::implements_interfaces,
                Rule::interface_type,
                Rule::fields_definition,
                Rule::field_definition,
                Rule::union_type,
                Rule::union_member_types,
                Rule::enum_type,
                Rule::enum_values,
                Rule::enum_value_definition,
                Rule::input_object_type,
                Rule::input_fields_definition,
                Rule::extend,
                Rule::directive_definition,
                Rule::repeatable,
                Rule::directive_locations,
                Rule::directive_location,
                Rule::arguments_definition,
                Rule::input_value_definition,
                Rule::generic_type_parameters,
                Rule::generic_type_parameter,
                Rule::generic_type_arguments,
                Rule::generic_type,
                Rule::where_clause,
                Rule::where_constraint,
                Rule::constraint_bound,
                Rule::operation_type,
                Rule::default_value,
                Rule::type_,
                Rule::type_base,
                Rule::list_type,
                Rule::set_type,
                Rule::map_type,
                Rule::const_value,
                Rule::value,
                Rule::variable,
                Rule::number,
                Rule::float,
                Rule::fractional,
                Rule::exponent,
                Rule::int,
                Rule::string,
                Rule::block_string_content,
                Rule::block_string_character,
                Rule::string_content,
                Rule::string_character,
                Rule::unicode_scalar_value_hex,
                Rule::boolean,
                Rule::null,
                Rule::enum_value,
                Rule::const_list,
                Rule::list,
                Rule::const_set,
                Rule::set,
                Rule::const_map,
                Rule::map,
                Rule::const_map_entry,
                Rule::map_entry,
                Rule::const_object,
                Rule::object,
                Rule::const_object_field,
                Rule::object_field,
                Rule::const_directives,
                Rule::directives,
                Rule::const_directive,
                Rule::directive,
                Rule::const_arguments,
                Rule::arguments,
                Rule::const_argument,
                Rule::argument,
                Rule::name_start,
                Rule::name,
            ]
        }
    }
    #[allow(clippy::all)]
    impl ::pest::Parser<Rule> for GrostParser {
        fn parse<'i>(
            rule: Rule,
            input: &'i str,
        ) -> ::std::result::Result<
            ::pest::iterators::Pairs<'i, Rule>,
            ::pest::error::Error<Rule>,
        > {
            mod rules {
                #![allow(clippy::upper_case_acronyms)]
                pub mod hidden {
                    use super::super::Rule;
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn skip(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        if state.atomicity() == ::pest::Atomicity::NonAtomic {
                            state
                                .sequence(|state| {
                                    state
                                        .repeat(|state| super::visible::WHITESPACE(state))
                                        .and_then(|state| {
                                            state
                                                .repeat(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            super::visible::COMMENT(state)
                                                                .and_then(|state| {
                                                                    state.repeat(|state| super::visible::WHITESPACE(state))
                                                                })
                                                        })
                                                })
                                        })
                                })
                        } else {
                            Ok(state)
                        }
                    }
                }
                pub mod visible {
                    use super::super::Rule;
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn WHITESPACE(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .atomic(
                                ::pest::Atomicity::Atomic,
                                |state| {
                                    state
                                        .match_string(" ")
                                        .or_else(|state| { state.match_string(",") })
                                        .or_else(|state| { state.match_string("\t") })
                                        .or_else(|state| { state.match_string("\u{feff}") })
                                        .or_else(|state| { self::line_terminator(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn COMMENT(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .atomic(
                                ::pest::Atomicity::Atomic,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("#")
                                                .and_then(|state| {
                                                    state
                                                        .repeat(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .lookahead(false, |state| { self::line_terminator(state) })
                                                                        .and_then(|state| { self::ANY(state) })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn line_terminator(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::line_terminator,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state
                                                    .match_string("\r\n")
                                                    .or_else(|state| { state.match_string("\r") })
                                                    .or_else(|state| { state.match_string("\n") })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn executable_document(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::executable_document,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::SOI(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::executable_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::executable_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::executable_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::EOI(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn executable_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::executable_definition,
                                |state| {
                                    self::operation_definition(state)
                                        .or_else(|state| { self::fragment_definition(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn operation_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::operation_definition,
                                |state| {
                                    self::named_operation_definition(state)
                                        .or_else(|state| { self::selection_set(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn named_operation_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::named_operation_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::operation_type(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::name(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::variable_definitions(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::selection_set(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn variable_definitions(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::variable_definitions,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("(")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::variable_definition(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::variable_definition(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(")") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn variable_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::variable_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::variable(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::default_value(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn selection_set(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::selection_set,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::selection(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::selection(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::selection(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn selection(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::selection,
                                |state| {
                                    self::field(state)
                                        .or_else(|state| { self::inline_fragment(state) })
                                        .or_else(|state| { self::fragment_spread(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn field(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::field,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::alias(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::arguments(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::selection_set(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn alias(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::alias,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn fragment_spread(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::fragment_spread,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("...")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .lookahead(false, |state| { self::type_condition(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn inline_fragment(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::inline_fragment,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("...")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::type_condition(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::selection_set(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn fragment_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::fragment_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("fragment")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_condition(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::selection_set(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn type_condition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .atomic(
                                ::pest::Atomicity::CompoundAtomic,
                                |state| {
                                    state
                                        .rule(
                                            Rule::type_condition,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .match_string("on")
                                                            .and_then(|state| {
                                                                state
                                                                    .sequence(|state| {
                                                                        self::WHITESPACE(state)
                                                                            .and_then(|state| {
                                                                                state.repeat(|state| { self::WHITESPACE(state) })
                                                                            })
                                                                    })
                                                            })
                                                            .and_then(|state| { self::name(state) })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn service_document(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::service_document,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::SOI(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::type_system_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::type_system_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::type_system_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::EOI(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn type_system_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::type_system_definition,
                                |state| {
                                    self::schema_definition(state)
                                        .or_else(|state| { self::type_definition(state) })
                                        .or_else(|state| { self::directive_definition(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn schema_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::schema_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("schema")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("{") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::operation_type_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::operation_type_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| {
                                                                                                                    self::operation_type_definition(state)
                                                                                                                })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("schema") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::const_directives(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { state.match_string("{") })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .sequence(|state| {
                                                                                    self::operation_type_definition(state)
                                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .sequence(|state| {
                                                                                                    state
                                                                                                        .optional(|state| {
                                                                                                            self::operation_type_definition(state)
                                                                                                                .and_then(|state| {
                                                                                                                    state
                                                                                                                        .repeat(|state| {
                                                                                                                            state
                                                                                                                                .sequence(|state| {
                                                                                                                                    super::hidden::skip(state)
                                                                                                                                        .and_then(|state| {
                                                                                                                                            self::operation_type_definition(state)
                                                                                                                                        })
                                                                                                                                })
                                                                                                                        })
                                                                                                                })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { state.match_string("}") })
                                                                })
                                                                .or_else(|state| { self::const_directives(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn operation_type_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::operation_type_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::operation_type(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn type_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::type_definition,
                                |state| {
                                    self::scalar_type(state)
                                        .or_else(|state| { self::object_type(state) })
                                        .or_else(|state| { self::interface_type(state) })
                                        .or_else(|state| { self::union_type(state) })
                                        .or_else(|state| { self::enum_type(state) })
                                        .or_else(|state| { self::input_object_type(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn scalar_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::scalar_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("scalar") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_parameters(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::where_clause(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("scalar") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::generic_type_parameters(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state.optional(|state| { self::where_clause(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::const_directives(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn object_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::object_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("type") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_parameters(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::implements_interfaces(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::where_clause(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::fields_definition(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("type") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::generic_type_parameters(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::implements_interfaces(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| {
                                                                            state.optional(|state| { self::where_clause(state) })
                                                                        })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .sequence(|state| {
                                                                                    state
                                                                                        .optional(|state| { self::const_directives(state) })
                                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                                        .and_then(|state| { self::fields_definition(state) })
                                                                                })
                                                                                .or_else(|state| { self::const_directives(state) })
                                                                        })
                                                                })
                                                                .or_else(|state| { self::implements_interfaces(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn implements_interfaces(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::implements_interfaces,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("implements")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { state.match_string("&") })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::generic_type(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string("&")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::generic_type(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string("&")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::generic_type(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn interface_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::interface_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("interface") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_parameters(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::implements_interfaces(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::where_clause(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::fields_definition(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("interface") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::generic_type_parameters(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::implements_interfaces(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state.optional(|state| { self::where_clause(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::const_directives(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { self::fields_definition(state) })
                                                                })
                                                                .or_else(|state| { self::const_directives(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn fields_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::fields_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::field_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::field_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::field_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn field_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::field_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::arguments_definition(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn union_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::union_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("union") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_parameters(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::where_clause(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::union_member_types(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("union") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::generic_type_parameters(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state.optional(|state| { self::where_clause(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::const_directives(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { self::union_member_types(state) })
                                                                })
                                                                .or_else(|state| { self::const_directives(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn union_member_types(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::union_member_types,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("=")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { state.match_string("|") })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::generic_type(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string("|")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::generic_type(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string("|")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::generic_type(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn enum_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::enum_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("enum") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::enum_values(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("enum") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::const_directives(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { self::enum_values(state) })
                                                                })
                                                                .or_else(|state| { self::const_directives(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn enum_values(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::enum_values,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::enum_value_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::enum_value_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::enum_value_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn enum_value_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::enum_value_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::enum_value(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn input_object_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::input_object_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("input") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_parameters(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::where_clause(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::input_fields_definition(state) })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    self::extend(state)
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { state.match_string("input") })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::name(state) })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .optional(|state| { self::generic_type_parameters(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state.optional(|state| { self::where_clause(state) })
                                                        })
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    state
                                                                        .optional(|state| { self::const_directives(state) })
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| { self::input_fields_definition(state) })
                                                                })
                                                                .or_else(|state| { self::const_directives(state) })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn input_fields_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::input_fields_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::input_value_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::input_value_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::input_value_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn extend(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(Rule::extend, |state| { state.match_string("extend") })
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn directive_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::directive_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("directive") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("@") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::arguments_definition(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::repeatable(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("on") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::directive_locations(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn repeatable(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::repeatable,
                                |state| {
                                    state.optional(|state| { state.match_string("repeatable") })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn directive_locations(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::directive_locations,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { state.match_string("|") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::directive_location(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string("|")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::directive_location(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string("|")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::directive_location(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn directive_location(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::directive_location,
                                |state| {
                                    state
                                        .match_string("QUERY")
                                        .or_else(|state| { state.match_string("MUTATION") })
                                        .or_else(|state| { state.match_string("SUBSCRIPTION") })
                                        .or_else(|state| { state.match_string("FIELD_DEFINITION") })
                                        .or_else(|state| { state.match_string("FIELD") })
                                        .or_else(|state| {
                                            state.match_string("FRAGMENT_DEFINITION")
                                        })
                                        .or_else(|state| { state.match_string("FRAGMENT_SPREAD") })
                                        .or_else(|state| { state.match_string("INLINE_FRAGMENT") })
                                        .or_else(|state| {
                                            state.match_string("VARIABLE_DEFINITION")
                                        })
                                        .or_else(|state| { state.match_string("SCHEMA") })
                                        .or_else(|state| { state.match_string("SCALAR") })
                                        .or_else(|state| { state.match_string("OBJECT") })
                                        .or_else(|state| {
                                            state.match_string("ARGUMENT_DEFINITION")
                                        })
                                        .or_else(|state| { state.match_string("INTERFACE") })
                                        .or_else(|state| { state.match_string("UNION") })
                                        .or_else(|state| { state.match_string("ENUM_VALUE") })
                                        .or_else(|state| { state.match_string("ENUM") })
                                        .or_else(|state| { state.match_string("INPUT_OBJECT") })
                                        .or_else(|state| {
                                            state.match_string("INPUT_FIELD_DEFINITION")
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn arguments_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::arguments_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("(")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::input_value_definition(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::input_value_definition(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::input_value_definition(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(")") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn input_value_definition(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::input_value_definition,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { self::string(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::default_value(state) })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_directives(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn generic_type_parameters(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::generic_type_parameters,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::generic_type_parameter(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string(",")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::generic_type_parameter(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string(",")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::generic_type_parameter(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn generic_type_parameter(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::generic_type_parameter,
                                |state| { self::name(state) },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn generic_type_arguments(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::generic_type_arguments,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string(",")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::type_(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string(",")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::type_(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn generic_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::generic_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_arguments(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn where_clause(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::where_clause,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("where")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::where_constraint(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string(",")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::where_constraint(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string(",")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::where_constraint(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn where_constraint(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::where_constraint,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::constraint_bound(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .match_string("+")
                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                .and_then(|state| { self::constraint_bound(state) })
                                                                        })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            state
                                                                                                                .match_string("+")
                                                                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                                                                .and_then(|state| { self::constraint_bound(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn constraint_bound(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::constraint_bound,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| { self::generic_type_arguments(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn operation_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::operation_type,
                                |state| {
                                    state
                                        .match_string("query")
                                        .or_else(|state| { state.match_string("mutation") })
                                        .or_else(|state| { state.match_string("subscription") })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn default_value(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::default_value,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("=")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::const_value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn type_(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::type_,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        self::type_base(state)
                                                            .and_then(|state| {
                                                                state.optional(|state| { state.match_string("!") })
                                                            })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn type_base(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::type_base,
                                |state| {
                                    self::list_type(state)
                                        .or_else(|state| { self::set_type(state) })
                                        .or_else(|state| { self::map_type(state) })
                                        .or_else(|state| { self::generic_type(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn list_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::list_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("[")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("]") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn set_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::set_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn map_type(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::map_type,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(",") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::type_(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_value(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_value,
                                |state| {
                                    self::number(state)
                                        .or_else(|state| { self::string(state) })
                                        .or_else(|state| { self::boolean(state) })
                                        .or_else(|state| { self::null(state) })
                                        .or_else(|state| { self::enum_value(state) })
                                        .or_else(|state| { self::const_list(state) })
                                        .or_else(|state| { self::const_set(state) })
                                        .or_else(|state| { self::const_map(state) })
                                        .or_else(|state| { self::const_object(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn value(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::value,
                                |state| {
                                    self::variable(state)
                                        .or_else(|state| { self::number(state) })
                                        .or_else(|state| { self::string(state) })
                                        .or_else(|state| { self::boolean(state) })
                                        .or_else(|state| { self::null(state) })
                                        .or_else(|state| { self::enum_value(state) })
                                        .or_else(|state| { self::list(state) })
                                        .or_else(|state| { self::set(state) })
                                        .or_else(|state| { self::map(state) })
                                        .or_else(|state| { self::object(state) })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn variable(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::variable,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("$")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn number(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::number,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        self::float(state)
                                                            .or_else(|state| { self::int(state) })
                                                            .and_then(|state| {
                                                                state.lookahead(false, |state| { self::name_start(state) })
                                                            })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn float(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::float,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::int(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::fractional(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| { self::exponent(state) })
                                                        })
                                                        .or_else(|state| { self::fractional(state) })
                                                        .or_else(|state| { self::exponent(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn fractional(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::fractional,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string(".")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_DIGIT(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::ASCII_DIGIT(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::ASCII_DIGIT(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn exponent(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::exponent,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("E")
                                                .or_else(|state| { state.match_string("e") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .optional(|state| {
                                                            state
                                                                .match_string("+")
                                                                .or_else(|state| { state.match_string("-") })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_DIGIT(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::ASCII_DIGIT(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::ASCII_DIGIT(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn int(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::int,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .optional(|state| { state.match_string("-") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .match_string("0")
                                                        .or_else(|state| {
                                                            state
                                                                .sequence(|state| {
                                                                    self::ASCII_NONZERO_DIGIT(state)
                                                                        .and_then(|state| { super::hidden::skip(state) })
                                                                        .and_then(|state| {
                                                                            state
                                                                                .sequence(|state| {
                                                                                    state
                                                                                        .optional(|state| {
                                                                                            self::ASCII_DIGIT(state)
                                                                                                .and_then(|state| {
                                                                                                    state
                                                                                                        .repeat(|state| {
                                                                                                            state
                                                                                                                .sequence(|state| {
                                                                                                                    super::hidden::skip(state)
                                                                                                                        .and_then(|state| { self::ASCII_DIGIT(state) })
                                                                                                                })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .atomic(
                                ::pest::Atomicity::CompoundAtomic,
                                |state| {
                                    state
                                        .rule(
                                            Rule::string,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .match_string("\"\"\"")
                                                            .and_then(|state| { self::block_string_content(state) })
                                                            .and_then(|state| { state.match_string("\"\"\"") })
                                                    })
                                                    .or_else(|state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .match_string("\"")
                                                                    .and_then(|state| { self::string_content(state) })
                                                                    .and_then(|state| { state.match_string("\"") })
                                                            })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn block_string_content(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::block_string_content,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state
                                                    .repeat(|state| { self::block_string_character(state) })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn block_string_character(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::block_string_character,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .lookahead(
                                                    false,
                                                    |state| {
                                                        state
                                                            .match_string("\"\"\"")
                                                            .or_else(|state| { state.match_string("\\\"\"\"") })
                                                    },
                                                )
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ANY(state) })
                                        })
                                        .or_else(|state| { state.match_string("\\\"\"\"") })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_content(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::string_content,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state.repeat(|state| { self::string_character(state) })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_character(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::string_character,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .lookahead(
                                                    false,
                                                    |state| {
                                                        state
                                                            .match_string("\"")
                                                            .or_else(|state| { state.match_string("\\") })
                                                            .or_else(|state| { self::line_terminator(state) })
                                                    },
                                                )
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ANY(state) })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("\\")
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| {
                                                            state
                                                                .match_string("\"")
                                                                .or_else(|state| { state.match_string("\\") })
                                                                .or_else(|state| { state.match_string("/") })
                                                                .or_else(|state| { state.match_string("b") })
                                                                .or_else(|state| { state.match_string("f") })
                                                                .or_else(|state| { state.match_string("n") })
                                                                .or_else(|state| { state.match_string("r") })
                                                                .or_else(|state| { state.match_string("t") })
                                                        })
                                                })
                                        })
                                        .or_else(|state| {
                                            state
                                                .sequence(|state| {
                                                    state
                                                        .match_string("\\u")
                                                        .and_then(|state| { super::hidden::skip(state) })
                                                        .and_then(|state| { self::unicode_scalar_value_hex(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn unicode_scalar_value_hex(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::unicode_scalar_value_hex,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .lookahead(
                                                    false,
                                                    |state| {
                                                        state
                                                            .sequence(|state| {
                                                                state
                                                                    .match_insensitive("d")
                                                                    .and_then(|state| { super::hidden::skip(state) })
                                                                    .and_then(|state| {
                                                                        state
                                                                            .match_range('8'..'9')
                                                                            .or_else(|state| { state.match_range('a'..'f') })
                                                                            .or_else(|state| { state.match_range('A'..'F') })
                                                                    })
                                                            })
                                                    },
                                                )
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_HEX_DIGIT(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_HEX_DIGIT(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_HEX_DIGIT(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::ASCII_HEX_DIGIT(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn boolean(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::boolean,
                                |state| {
                                    state
                                        .match_string("true")
                                        .or_else(|state| { state.match_string("false") })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn null(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.rule(Rule::null, |state| { state.match_string("null") })
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn enum_value(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .atomic(
                                ::pest::Atomicity::CompoundAtomic,
                                |state| {
                                    state
                                        .rule(
                                            Rule::enum_value,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        state
                                                            .lookahead(
                                                                false,
                                                                |state| {
                                                                    self::boolean(state).or_else(|state| { self::null(state) })
                                                                },
                                                            )
                                                            .and_then(|state| { self::name(state) })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_list(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_list,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("[")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::const_value(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::const_value(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("]") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn list(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::list,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("[")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::value(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::value(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("]") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_set(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_set,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::const_value(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::const_value(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn set(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::set,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::value(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::value(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_map(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_map,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::const_map_entry(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::const_map_entry(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn map(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::map,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("<")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::map_entry(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::map_entry(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(">") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_map_entry(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_map_entry,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::const_value(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::const_value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn map_entry(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::map_entry,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::value(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_object(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_object,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::const_object_field(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::const_object_field(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn object(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::object,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("{")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::object_field(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::object_field(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string("}") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_object_field(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_object_field,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::const_value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn object_field(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::object_field,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_directives(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_directives,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::const_directive(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::const_directive(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::const_directive(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn directives(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::directives,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::directive(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            state
                                                                .optional(|state| {
                                                                    self::directive(state)
                                                                        .and_then(|state| {
                                                                            state
                                                                                .repeat(|state| {
                                                                                    state
                                                                                        .sequence(|state| {
                                                                                            super::hidden::skip(state)
                                                                                                .and_then(|state| { self::directive(state) })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_directive(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_directive,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("@")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::const_arguments(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn directive(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::directive,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("@")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::name(state) })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state.optional(|state| { self::arguments(state) })
                                                })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_arguments(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_arguments,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("(")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::const_argument(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::const_argument(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::const_argument(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(")") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn arguments(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::arguments,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            state
                                                .match_string("(")
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| {
                                                    state
                                                        .sequence(|state| {
                                                            self::argument(state)
                                                                .and_then(|state| { super::hidden::skip(state) })
                                                                .and_then(|state| {
                                                                    state
                                                                        .sequence(|state| {
                                                                            state
                                                                                .optional(|state| {
                                                                                    self::argument(state)
                                                                                        .and_then(|state| {
                                                                                            state
                                                                                                .repeat(|state| {
                                                                                                    state
                                                                                                        .sequence(|state| {
                                                                                                            super::hidden::skip(state)
                                                                                                                .and_then(|state| { self::argument(state) })
                                                                                                        })
                                                                                                })
                                                                                        })
                                                                                })
                                                                        })
                                                                })
                                                        })
                                                })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(")") })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn const_argument(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::const_argument,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::const_value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn argument(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::argument,
                                |state| {
                                    state
                                        .sequence(|state| {
                                            self::name(state)
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { state.match_string(":") })
                                                .and_then(|state| { super::hidden::skip(state) })
                                                .and_then(|state| { self::value(state) })
                                        })
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn name_start(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::name_start,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                self::ASCII_ALPHA(state)
                                                    .or_else(|state| { state.match_string("_") })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn name(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .rule(
                                Rule::name,
                                |state| {
                                    state
                                        .atomic(
                                            ::pest::Atomicity::Atomic,
                                            |state| {
                                                state
                                                    .sequence(|state| {
                                                        self::name_start(state)
                                                            .and_then(|state| {
                                                                state
                                                                    .repeat(|state| {
                                                                        self::ASCII_ALPHA(state)
                                                                            .or_else(|state| { self::ASCII_DIGIT(state) })
                                                                            .or_else(|state| { state.match_string("_") })
                                                                    })
                                                            })
                                                    })
                                            },
                                        )
                                },
                            )
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ANY(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.skip(1)
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn EOI(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.rule(Rule::EOI, |state| state.end_of_input())
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn SOI(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.start_of_input()
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_DIGIT(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.match_range('0'..'9')
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_NONZERO_DIGIT(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state.match_range('1'..'9')
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_HEX_DIGIT(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .match_range('0'..'9')
                            .or_else(|state| state.match_range('a'..'f'))
                            .or_else(|state| state.match_range('A'..'F'))
                    }
                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_ALPHA(
                        state: ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    ) -> ::pest::ParseResult<
                        ::std::boxed::Box<::pest::ParserState<'_, Rule>>,
                    > {
                        state
                            .match_range('a'..'z')
                            .or_else(|state| state.match_range('A'..'Z'))
                    }
                }
                pub use self::visible::*;
            }
            ::pest::state(
                input,
                |state| {
                    match rule {
                        Rule::WHITESPACE => rules::WHITESPACE(state),
                        Rule::COMMENT => rules::COMMENT(state),
                        Rule::line_terminator => rules::line_terminator(state),
                        Rule::executable_document => rules::executable_document(state),
                        Rule::executable_definition => {
                            rules::executable_definition(state)
                        }
                        Rule::operation_definition => rules::operation_definition(state),
                        Rule::named_operation_definition => {
                            rules::named_operation_definition(state)
                        }
                        Rule::variable_definitions => rules::variable_definitions(state),
                        Rule::variable_definition => rules::variable_definition(state),
                        Rule::selection_set => rules::selection_set(state),
                        Rule::selection => rules::selection(state),
                        Rule::field => rules::field(state),
                        Rule::alias => rules::alias(state),
                        Rule::fragment_spread => rules::fragment_spread(state),
                        Rule::inline_fragment => rules::inline_fragment(state),
                        Rule::fragment_definition => rules::fragment_definition(state),
                        Rule::type_condition => rules::type_condition(state),
                        Rule::service_document => rules::service_document(state),
                        Rule::type_system_definition => {
                            rules::type_system_definition(state)
                        }
                        Rule::schema_definition => rules::schema_definition(state),
                        Rule::operation_type_definition => {
                            rules::operation_type_definition(state)
                        }
                        Rule::type_definition => rules::type_definition(state),
                        Rule::scalar_type => rules::scalar_type(state),
                        Rule::object_type => rules::object_type(state),
                        Rule::implements_interfaces => {
                            rules::implements_interfaces(state)
                        }
                        Rule::interface_type => rules::interface_type(state),
                        Rule::fields_definition => rules::fields_definition(state),
                        Rule::field_definition => rules::field_definition(state),
                        Rule::union_type => rules::union_type(state),
                        Rule::union_member_types => rules::union_member_types(state),
                        Rule::enum_type => rules::enum_type(state),
                        Rule::enum_values => rules::enum_values(state),
                        Rule::enum_value_definition => {
                            rules::enum_value_definition(state)
                        }
                        Rule::input_object_type => rules::input_object_type(state),
                        Rule::input_fields_definition => {
                            rules::input_fields_definition(state)
                        }
                        Rule::extend => rules::extend(state),
                        Rule::directive_definition => rules::directive_definition(state),
                        Rule::repeatable => rules::repeatable(state),
                        Rule::directive_locations => rules::directive_locations(state),
                        Rule::directive_location => rules::directive_location(state),
                        Rule::arguments_definition => rules::arguments_definition(state),
                        Rule::input_value_definition => {
                            rules::input_value_definition(state)
                        }
                        Rule::generic_type_parameters => {
                            rules::generic_type_parameters(state)
                        }
                        Rule::generic_type_parameter => {
                            rules::generic_type_parameter(state)
                        }
                        Rule::generic_type_arguments => {
                            rules::generic_type_arguments(state)
                        }
                        Rule::generic_type => rules::generic_type(state),
                        Rule::where_clause => rules::where_clause(state),
                        Rule::where_constraint => rules::where_constraint(state),
                        Rule::constraint_bound => rules::constraint_bound(state),
                        Rule::operation_type => rules::operation_type(state),
                        Rule::default_value => rules::default_value(state),
                        Rule::type_ => rules::type_(state),
                        Rule::type_base => rules::type_base(state),
                        Rule::list_type => rules::list_type(state),
                        Rule::set_type => rules::set_type(state),
                        Rule::map_type => rules::map_type(state),
                        Rule::const_value => rules::const_value(state),
                        Rule::value => rules::value(state),
                        Rule::variable => rules::variable(state),
                        Rule::number => rules::number(state),
                        Rule::float => rules::float(state),
                        Rule::fractional => rules::fractional(state),
                        Rule::exponent => rules::exponent(state),
                        Rule::int => rules::int(state),
                        Rule::string => rules::string(state),
                        Rule::block_string_content => rules::block_string_content(state),
                        Rule::block_string_character => {
                            rules::block_string_character(state)
                        }
                        Rule::string_content => rules::string_content(state),
                        Rule::string_character => rules::string_character(state),
                        Rule::unicode_scalar_value_hex => {
                            rules::unicode_scalar_value_hex(state)
                        }
                        Rule::boolean => rules::boolean(state),
                        Rule::null => rules::null(state),
                        Rule::enum_value => rules::enum_value(state),
                        Rule::const_list => rules::const_list(state),
                        Rule::list => rules::list(state),
                        Rule::const_set => rules::const_set(state),
                        Rule::set => rules::set(state),
                        Rule::const_map => rules::const_map(state),
                        Rule::map => rules::map(state),
                        Rule::const_map_entry => rules::const_map_entry(state),
                        Rule::map_entry => rules::map_entry(state),
                        Rule::const_object => rules::const_object(state),
                        Rule::object => rules::object(state),
                        Rule::const_object_field => rules::const_object_field(state),
                        Rule::object_field => rules::object_field(state),
                        Rule::const_directives => rules::const_directives(state),
                        Rule::directives => rules::directives(state),
                        Rule::const_directive => rules::const_directive(state),
                        Rule::directive => rules::directive(state),
                        Rule::const_arguments => rules::const_arguments(state),
                        Rule::arguments => rules::arguments(state),
                        Rule::const_argument => rules::const_argument(state),
                        Rule::argument => rules::argument(state),
                        Rule::name_start => rules::name_start(state),
                        Rule::name => rules::name(state),
                        Rule::EOI => rules::EOI(state),
                    }
                },
            )
        }
    }
}
