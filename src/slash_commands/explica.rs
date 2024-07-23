use serenity::all::{CommandDataOption, CommandOptionType, CreateCommand, CreateCommandOption};
use std::path::PathBuf;

pub fn register() -> CreateCommand {
    CreateCommand::new("explica")
        .description("Explica un concepto de Rust")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "concepto",
                "Este sera el concepto que se explicara",
            )
            .kind(CommandOptionType::String)
            .required(true)
            .add_string_choice("arrays", "arrays")
            .add_string_choice("borrowing", "borrowing")
            .add_string_choice("closures", "closures")
            .add_string_choice("condicionales", "condicionales")
            .add_string_choice("constantes", "constantes")
            .add_string_choice("enums", "enums")
            .add_string_choice("for", "for")
            .add_string_choice("funciones", "funciones")
            .add_string_choice("generics", "generics")
            .add_string_choice("if_let", "if_let")
            .add_string_choice("iterators", "iterators")
            .add_string_choice("let_else", "let_else")
            .add_string_choice("lifetimes", "lifetimes")
            .add_string_choice("loop", "loop")
            .add_string_choice("macros", "macros")
            .add_string_choice("match", "match")
            .add_string_choice("metodos", "metodos")
            .add_string_choice("modulos", "modulos")
            .add_string_choice("operadores", "operadores")
            .add_string_choice("ownership", "ownership")
            .add_string_choice("result", "result")
            .add_string_choice("return", "return")
            .add_string_choice("scopes", "scopes")
            .add_string_choice("shadowing", "shadowing")
            .add_string_choice("slices", "slices")
            // .add_string_choice("string", "string")
            // .add_string_choice("struct", "struct")
            // .add_string_choice("tipo_de_datos", "tipo_de_datos")
            // .add_string_choice("traits", "traits")
            // .add_string_choice("tuplas", "tuplas")
            // .add_string_choice("variables", "variables")
            // .add_string_choice("vectores", "vectores")
            // .add_string_choice("while", "while"),
        )
}

pub fn run(options: &[CommandDataOption]) -> String {
    let mut concept = None;
    for option in options {
        if option.name == "concepto_1" {
            concept = option.value.as_str();
        }
        if option.name == "concepto_2" {
            concept = option.value.as_str();
        }
    }
    let concepts_folder = PathBuf::from("static/rust-examples/docs");

    let Some(concept) = concept else {
        return "No se ha encontrado el concepto".to_string();
    };

    let concept = concept.to_lowercase() + ".md";
    std::fs::read_to_string(concepts_folder.join(concept))
        .unwrap_or("No se ha encontrado el concepto".to_string())
}
