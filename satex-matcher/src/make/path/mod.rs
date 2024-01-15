use std::collections::HashMap;

use path_tree::{PathTree, Piece, Position};

pub use make::MakePathMatcher;
use satex_core::apply::Apply;
use satex_core::essential::{Essential, PathVariables};
use satex_core::{satex_error, Error};

use crate::RouteMatcher;

mod make;

pub struct PathMatcher {
    tree: PathTree<()>,
}

impl PathMatcher {
    pub fn new(patterns: Vec<String>) -> Self {
        let mut tree = PathTree::new();
        patterns.iter().for_each(|pattern| {
            let _ = tree.insert(pattern, ());
        });
        Self { tree }
    }
}

impl RouteMatcher for PathMatcher {
    fn is_match(&self, essential: &mut Essential) -> Result<bool, Error> {
        let path = essential.uri.path();
        match self.tree.find(path) {
            Some((_, path)) => {
                let names = path
                    .pieces
                    .iter()
                    .flat_map(|piece| match piece {
                        Piece::String(_) => None,
                        Piece::Parameter(position, _) => Some(position),
                    })
                    .map(|position| match position {
                        Position::Index(_, name) => name,
                        Position::Named(name) => name,
                    })
                    .try_fold(vec![], |names, name| {
                        match String::from_utf8(name.to_vec()) {
                            Ok(name) => Ok(names.apply(|names| names.push(name))),
                            Err(e) => Err(satex_error!(e)),
                        }
                    })?;
                let raws = path.raws;
                if raws.len() != names.len() {
                    Err(satex_error!(
                        "Expecting path variable size: {}, but actual size: {}",
                        raws.len(),
                        names.len()
                    ))
                } else {
                    let variables = names
                        .into_iter()
                        .zip(raws.into_iter())
                        .map(|(name, value)| (name, value.to_string()))
                        .collect::<HashMap<_, _>>();
                    essential.extensions.insert(PathVariables(variables));
                    Ok(true)
                }
            }
            None => Ok(false),
        }
    }
}
