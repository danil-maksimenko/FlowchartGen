use std::path::PathBuf;

use crate::{ast::Ast, to_usize, error::{ParseError, CodeError}};

use clang::{
    Clang, Entity, EntityKind, Index, TranslationUnit,
};

#[tauri::command]
pub fn parse_cpp_file(file_name: PathBuf) -> Result<Vec<Ast>, ParseError> {
    let file_content = std::fs::read_to_string(&file_name).unwrap(); // todo!!!!

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, false, true);
    let unit = index.parser(file_name).parse().unwrap();

    if let Some(errors) = get_errors(&unit) {
        return Err(errors.into());
    }

    let main_function: Entity<'_> = unit
        .get_entity()
        .get_children()
        .into_iter()
        .find(|e| {
            if let Some(name) = e.get_name() {
                if name == "main" && e.get_kind() == EntityKind::FunctionDecl {
                    return true;
                }
            }
            false
        })
        .ok_or(ParseError::NoMain)?;

    let mut ast_vector = vec![];

    visit(main_function, &file_content, &mut ast_vector);

    remove_duplicates_from_tree(&mut ast_vector);

    Ok(ast_vector)
}

fn code_of_entity<'a>(entity: Entity<'_>, file_content: &'a str) -> &'a str {
    let range = entity.get_range().unwrap();
    let start = to_usize!(range.get_start().get_file_location().offset);
    let end = to_usize!(range.get_end().get_file_location().offset);
    &file_content[start..end]
}

fn visit(entity: Entity<'_>, file_content: &str, ast_tree: &mut Vec<Ast>) {
    for child in entity.get_children() {
        let kind = entity.get_kind();
        let code = code_of_entity(entity, file_content);
        match kind {
            EntityKind::VarDecl | EntityKind::CallExpr => ast_tree.push(Ast::identify_in_out_or_action(code)),
            EntityKind::IfStmt => {
                visit_if_body(entity, file_content, ast_tree);
                continue;
            }
            _ => (),
        }
        visit(child, file_content, ast_tree);
    }
}

fn visit_if_body(entity: Entity<'_>, file_content: &str, buffer: &mut Vec<Ast>) {
    let mut body = vec![];
    let code = code_of_entity(entity.get_child(0).unwrap(), file_content);
    let condition = Ast::Action(code.into());
    let entity_body = entity.get_child(1).unwrap();
    visit(entity_body, file_content, &mut body);
    buffer.push(Ast::condition(condition, body));
}

// For some reasone, clang's parser duplicate function calls, if statements, etc.
fn remove_duplicates_from_tree(ast_tree: &mut Vec<Ast>) {
    for i in 0..ast_tree.len() {
        if let Some(ast) = ast_tree.get_mut(i) {
            match ast {
                Ast::Conditional { ref mut body, .. } => {
                    remove_duplicates_from_tree(body);
                    ast_tree.remove(i + 1);
                }
                Ast::Output(_) | Ast::Input(_) => {
                    ast_tree.remove(i + 1);
                }
                _ => (),
            }
        };
    }
}

fn get_errors(unit: &TranslationUnit) -> Option<Vec<CodeError>> {
    let errors: Vec<_> = unit
        .get_diagnostics()
        .into_iter()
        .filter_map(|diagnostic| diagnostic.try_into().ok())
        .collect();
    if ! errors.is_empty() {
        Some(errors)
    } else {
        None
    }
}