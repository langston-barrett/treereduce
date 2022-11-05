//! Read tree-sitter's `node-types.json`.
//
// Copied in part from [treeedbgen].
//
// [treeedbgen]: https://github.com/langston-barrett/treeedb/blob/1a2fae3509c76cd5a8e1004f808ea800d49d1a19/treeedbgen/src/lib.rs

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// node-types.json
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct Node {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    ty: String,
    named: bool,
    #[serde(default)] // empty
    children: Children,
    #[serde(default)] // empty
    fields: HashMap<String, Field>,
    #[serde(default)] // empty
    subtypes: Vec<Subtype>,
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct Children {
    multiple: bool,
    required: bool,
    types: Vec<Subtype>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct Field {
    multiple: bool,
    required: bool,
    types: Vec<Subtype>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct Subtype {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    ty: String,
    named: bool,
}

#[derive(Clone, Debug)]
pub struct FieldInfo {
    parent_ty: String,
    multiple: bool,
    required: bool,
}

#[derive(Clone, Debug)]
pub struct NodeTypes {
    children: HashMap<String, Children>,
    subtypes: HashMap<String, Vec<String>>,
    reverse_fields: HashMap<String, Vec<FieldInfo>>,
}

fn subtypes(name: &str, nodes: &Vec<Node>) -> Vec<String> {
    let mut r = vec![name.to_string()];
    for n in nodes {
        if n.ty == name {
            for subty in &n.subtypes {
                r.push(subty.ty.clone());
                r.extend(subtypes(&subty.ty, nodes));
            }
        }
    }
    r
}

// TODO(lb): Check if a field is a list, if so, try deleting all
impl NodeTypes {
    pub fn new(node_types_json_str: &str) -> Result<Self, serde_json::Error> {
        let nodes: Vec<Node> = serde_json::from_str(node_types_json_str)?;
        let subtypes: HashMap<_, _> = nodes
            .iter()
            .map(|n| (n.ty.clone(), subtypes(&n.ty, &nodes)))
            .collect();
        let mut reverse_fields = HashMap::new();

        // For each type of node...
        for node in &nodes {
            // Loop through it's fields...
            for (_field_name, field) in node.fields.iter() {
                // And save the name of all types that the field could be.
                for subtype in &field.types {
                    for subsubty in subtypes.get(&subtype.ty).unwrap_or(&Vec::new()) {
                        let entry = reverse_fields.entry(subsubty.clone());
                        entry
                            .and_modify(|v: &mut Vec<FieldInfo>| {
                                v.push(FieldInfo {
                                    parent_ty: node.ty.clone(),
                                    multiple: field.multiple,
                                    required: field.required,
                                });
                            })
                            .or_insert_with(|| {
                                vec![FieldInfo {
                                    parent_ty: node.ty.clone(),
                                    multiple: field.multiple,
                                    required: field.required,
                                }]
                            });
                    }
                }
            }
        }
        Ok(NodeTypes {
            children: nodes
                .iter()
                .map(|n| (n.ty.clone(), n.children.clone()))
                .collect(),
            subtypes,
            reverse_fields,
        })
    }

    /// Defaults to `true` if the real answer can't be determined.
    fn optional(&self, node_kind: &str, parent_kind: &str) -> bool {
        if let Some(flds) = self.reverse_fields.get(node_kind) {
            for fi in flds {
                if parent_kind == fi.parent_ty && (!fi.multiple || fi.required) {
                    return false;
                }
            }
        }
        true
    }

    /// Defaults to `true` if the real answer can't be determined.
    pub fn optional_node(&self, node: &tree_sitter::Node) -> bool {
        if let Some(p) = node.parent() {
            self.optional(node.kind(), p.kind())
        } else {
            true
        }
    }

    // TODO(lb): Also include fields that are multiple and not required
    // TODO(lb): Benchmark including fields that *are* required, defaulting to
    // the first
    pub fn list_types(&self, node: &tree_sitter::Node) -> Vec<String> {
        let mut kinds = Vec::new();
        if let Some(children) = self.children.get(node.kind()) {
            if children.multiple && !children.required {
                for child in &children.types {
                    kinds.push(child.ty.clone());
                }
            }
        }
        kinds
    }

    pub fn subtypes(&self, kind: &String) -> &[String] {
        self.subtypes.get(kind).expect("Invalid node kind")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional() {
        let nt = NodeTypes::new(tree_sitter_c::NODE_TYPES).unwrap();
        assert!(nt.optional("_expression", "return_statement"));
        assert!(!nt.optional("compound_statement", "function_definition"));
    }
}
