use crate::model::Project;

// TODO: this is all going into a generic metaprogramming library
pub struct TablePadding {
    id: usize,
    name: usize,
    created: usize,
    updated: usize,
}

impl TablePadding {
    pub fn default_padding(max_name: usize) -> Self {
        TablePadding {
            id: 4,
            name: max_name,
            created: 26,
            updated: 26,
        }
    }
}

pub fn print_table(padding: TablePadding, projs: Vec<Project>) {
    println!(
        "{:<width_id$} {:<width_name$} {:<width_created$} {:<width_updated$}",
        "ID",
        "NAME",
        "CREATED",
        "UPDATED",
        width_id = padding.id,
        width_name = padding.name,
        width_created = padding.created,
        width_updated = padding.updated
    );

    for proj in &projs {
        println!(
            "{:<width_id$} {:<width_name$} {:<width_created$} {:<width_updated$}",
            &proj.id,
            &proj.name,
            &proj.created,
            &proj.updated,
            width_id = padding.id,
            width_name = padding.name,
            width_created = padding.created,
            width_updated = padding.updated
        );
    }
}
