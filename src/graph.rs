use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    error::ConvertError,
    unit::{Conversion, chain_conversions},
};

/// A dimension groups units that are mutually convertible (e.g. Mass, Length).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dimension(pub &'static str);

/// Metadata about a single unit.
#[derive(Debug, Clone)]
pub struct UnitInfo {
    /// Canonical name (e.g. "kilogram")
    pub name: &'static str,
    /// Short symbol and alternate spellings (e.g. ["kg", "kilogram", "kilograms"])
    pub aliases: &'static [&'static str],
    /// Which dimension this unit belongs to
    pub dimension: Dimension,
}

/// A directed edge in the conversion graph.
#[derive(Debug, Clone)]
struct Edge {
    to: &'static str, // canonical name of target unit
    conversion: Conversion,
}

/// The static unit graph.
/// Nodes are units identified by their canonical name.
/// Edges are stored as adjacency lists.
pub struct UnitGraph {
    /// canonical name -> unit info
    units: HashMap<&'static str, UnitInfo>,
    /// alias/symbol -> canonical name
    alias_map: HashMap<&'static str, &'static str>,
    /// canonical name -> list of outgoing edges
    edges: HashMap<&'static str, Vec<Edge>>,
}

impl UnitGraph {
    /// Build an empty graph.
    pub fn new() -> Self {
        UnitGraph {
            units: HashMap::new(),
            alias_map: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    /// Register a unit. Call this before adding edges.
    pub fn add_unit(&mut self, info: UnitInfo) {
        for alias in info.aliases {
            self.alias_map.insert(alias, info.name);
        }
        self.alias_map.insert(info.name, info.name);
        self.units.insert(info.name, info);
    }

    /// Add a bidirectional edge between two canonical unit names.
    pub fn add_edge(&mut self, from: &'static str, to: &'static str, conversion: Conversion) {
        let inverse = conversion.inverse();
        self.edges
            .entry(from)
            .or_default()
            .push(Edge { to, conversion });
        self.edges.entry(to).or_default().push(Edge {
            to: from,
            conversion: inverse,
        });
    }

    /// Resolve an alias/symbol to a canonical unit name.
    pub fn resolve(&self, name: &str) -> Option<&'static str> {
        // case-insensitive lookup
        let lower = name.to_lowercase();
        self.alias_map
            .iter()
            .find(|(k, _)| k.to_lowercase() == lower)
            .map(|(_, v)| *v)
    }

    /// Convert `value` from `from_unit` to `to_unit`.
    /// Units can be given as any alias or canonical name.
    pub fn convert(&self, value: f64, from: &str, to: &str) -> Result<f64, ConvertError> {
        let from_canon = self
            .resolve(from)
            .ok_or_else(|| ConvertError::UnknownUnit(from.to_string()))?;
        let to_canon = self
            .resolve(to)
            .ok_or_else(|| ConvertError::UnknownUnit(to.to_string()))?;

        if from_canon == to_canon {
            return Ok(value);
        }

        // Check dimensions match
        let from_dim = &self.units[from_canon].dimension;
        let to_dim = &self.units[to_canon].dimension;
        if from_dim != to_dim {
            return Err(ConvertError::DimensionMismatch(
                from_dim.0.to_string(),
                to_dim.0.to_string(),
            ));
        }

        // BFS to find shortest path
        let path = self
            .bfs(from_canon, to_canon)
            .ok_or_else(|| ConvertError::NoPathFound(from.to_string(), to.to_string()))?;

        Ok(chain_conversions(value, &path))
    }

    /// BFS from `start` to `goal`, returning the chain of conversions to apply.
    fn bfs(&self, start: &'static str, goal: &'static str) -> Option<Vec<Conversion>> {
        // Each queue entry: (current_node, conversions_so_far)
        let mut queue: VecDeque<(&'static str, Vec<Conversion>)> = VecDeque::new();
        let mut visited: HashSet<&'static str> = HashSet::new();

        queue.push_back((start, vec![]));
        visited.insert(start);

        while let Some((node, path)) = queue.pop_front() {
            if let Some(edges) = self.edges.get(node) {
                for edge in edges {
                    if visited.contains(edge.to) {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.push(edge.conversion.clone());

                    if edge.to == goal {
                        return Some(new_path);
                    }

                    visited.insert(edge.to);
                    queue.push_back((edge.to, new_path));
                }
            }
        }
        None
    }

    /// List all known unit names and aliases.
    pub fn known_units(&self) -> Vec<&'static str> {
        let mut keys: Vec<_> = self.alias_map.keys().copied().collect();
        keys.sort_unstable();
        keys
    }
}

impl Default for UnitGraph {
    fn default() -> Self {
        Self::new()
    }
}
