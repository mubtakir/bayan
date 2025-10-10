//! Package management system for AlBayan
//! 
//! Handles package definition, versioning, and dependency management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

/// Package manifest (similar to Cargo.toml or package.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// Package metadata
    pub package: PackageInfo,
    /// Dependencies
    pub dependencies: HashMap<String, DependencySpec>,
    /// Development dependencies
    pub dev_dependencies: HashMap<String, DependencySpec>,
    /// Build dependencies
    pub build_dependencies: HashMap<String, DependencySpec>,
    /// Features
    pub features: HashMap<String, Vec<String>>,
    /// Build configuration
    pub build: Option<BuildConfig>,
}

/// Package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: Option<String>,
    /// Authors
    pub authors: Vec<String>,
    /// License
    pub license: Option<String>,
    /// Repository URL
    pub repository: Option<String>,
    /// Homepage URL
    pub homepage: Option<String>,
    /// Keywords
    pub keywords: Vec<String>,
    /// Categories
    pub categories: Vec<String>,
    /// Main entry point
    pub main: Option<String>,
}

/// Dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    /// Simple version requirement
    Version(String),
    /// Detailed dependency specification
    Detailed {
        /// Version requirement
        version: Option<String>,
        /// Git repository
        git: Option<String>,
        /// Git branch
        branch: Option<String>,
        /// Git tag
        tag: Option<String>,
        /// Git revision
        rev: Option<String>,
        /// Local path
        path: Option<String>,
        /// Optional dependency
        optional: Option<bool>,
        /// Default features
        default_features: Option<bool>,
        /// Features to enable
        features: Option<Vec<String>>,
    },
}

/// Build configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Build script
    pub script: Option<String>,
    /// Include patterns
    pub include: Vec<String>,
    /// Exclude patterns
    pub exclude: Vec<String>,
}

/// Package registry for managing packages
#[derive(Debug)]
pub struct PackageRegistry {
    /// Registered packages
    packages: HashMap<String, PackageManifest>,
    /// Package cache directory
    cache_dir: PathBuf,
    /// Registry URLs
    registries: Vec<String>,
}

impl PackageRegistry {
    /// Create a new package registry
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            cache_dir: PathBuf::from("~/.albayan/cache"),
            registries: vec![
                "https://packages.albayan.dev".to_string(),
            ],
        }
    }
    
    /// Register a package
    pub fn register_package(&mut self, manifest: PackageManifest) -> Result<()> {
        let package_key = format!("{}@{}", manifest.package.name, manifest.package.version);
        self.packages.insert(package_key, manifest);
        Ok(())
    }
    
    /// Find a package by name and version
    pub fn find_package(&self, name: &str, version: &str) -> Option<&PackageManifest> {
        let package_key = format!("{}@{}", name, version);
        self.packages.get(&package_key)
    }
    
    /// Resolve package dependencies
    pub fn resolve_dependencies(&self, manifest: &PackageManifest) -> Result<DependencyGraph> {
        let mut graph = DependencyGraph::new();
        let mut to_resolve = vec![(manifest.package.name.clone(), manifest.package.version.clone())];
        let mut resolved = HashMap::new();
        
        while let Some((name, version)) = to_resolve.pop() {
            if resolved.contains_key(&name) {
                continue;
            }
            
            let package = self.find_package(&name, &version)
                .ok_or_else(|| anyhow!("Package not found: {}@{}", name, version))?;
            
            // Add to graph
            graph.add_package(package.clone());
            resolved.insert(name.clone(), version.clone());
            
            // Add dependencies to resolve queue
            for (dep_name, dep_spec) in &package.dependencies {
                let dep_version = self.resolve_version_spec(dep_name, dep_spec)?;
                graph.add_dependency(&name, dep_name, &dep_version);
                
                if !resolved.contains_key(dep_name) {
                    to_resolve.push((dep_name.clone(), dep_version));
                }
            }
        }
        
        Ok(graph)
    }
    
    /// Resolve version specification to concrete version
    fn resolve_version_spec(&self, name: &str, spec: &DependencySpec) -> Result<String> {
        match spec {
            DependencySpec::Version(version) => {
                // For now, just return the version as-is
                // TODO: Implement semantic version resolution
                Ok(version.clone())
            }
            DependencySpec::Detailed { version, git, path, .. } => {
                if let Some(version) = version {
                    Ok(version.clone())
                } else if git.is_some() {
                    // TODO: Resolve git dependency
                    Ok("git".to_string())
                } else if path.is_some() {
                    // TODO: Resolve path dependency
                    Ok("path".to_string())
                } else {
                    Err(anyhow!("Invalid dependency specification for {}", name))
                }
            }
        }
    }
    
    /// Download and install a package
    pub fn install_package(&mut self, name: &str, version: &str) -> Result<()> {
        // TODO: Implement package download and installation
        println!("Installing package: {}@{}", name, version);
        Ok(())
    }
    
    /// Update package cache
    pub fn update_cache(&mut self) -> Result<()> {
        // TODO: Implement cache update from registries
        println!("Updating package cache...");
        Ok(())
    }
}

impl Default for PackageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Dependency graph for package resolution
#[derive(Debug)]
pub struct DependencyGraph {
    /// Packages in the graph
    packages: HashMap<String, PackageManifest>,
    /// Dependencies between packages
    dependencies: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    /// Add a package to the graph
    pub fn add_package(&mut self, package: PackageManifest) {
        let package_key = format!("{}@{}", package.package.name, package.package.version);
        self.packages.insert(package_key.clone(), package);
        self.dependencies.entry(package_key).or_insert_with(Vec::new);
    }
    
    /// Add a dependency relationship
    pub fn add_dependency(&mut self, from: &str, to: &str, version: &str) {
        let from_key = from.to_string();
        let to_key = format!("{}@{}", to, version);
        
        self.dependencies
            .entry(from_key)
            .or_insert_with(Vec::new)
            .push(to_key);
    }
    
    /// Get topological order of packages
    pub fn topological_order(&self) -> Result<Vec<String>> {
        let mut visited = HashMap::new();
        let mut temp_visited = HashMap::new();
        let mut result = Vec::new();
        
        for package_key in self.packages.keys() {
            if !visited.contains_key(package_key) {
                self.visit(package_key, &mut visited, &mut temp_visited, &mut result)?;
            }
        }
        
        Ok(result)
    }
    
    /// Visit node in topological sort
    fn visit(
        &self,
        package_key: &str,
        visited: &mut HashMap<String, bool>,
        temp_visited: &mut HashMap<String, bool>,
        result: &mut Vec<String>,
    ) -> Result<()> {
        if temp_visited.get(package_key) == Some(&true) {
            return Err(anyhow!("Circular dependency detected involving {}", package_key));
        }
        
        if visited.get(package_key) == Some(&true) {
            return Ok(());
        }
        
        temp_visited.insert(package_key.to_string(), true);
        
        if let Some(deps) = self.dependencies.get(package_key) {
            for dep in deps {
                self.visit(dep, visited, temp_visited, result)?;
            }
        }
        
        temp_visited.insert(package_key.to_string(), false);
        visited.insert(package_key.to_string(), true);
        result.push(package_key.to_string());
        
        Ok(())
    }
    
    /// Check for circular dependencies
    pub fn has_cycles(&self) -> bool {
        self.topological_order().is_err()
    }
    
    /// Get all packages
    pub fn packages(&self) -> &HashMap<String, PackageManifest> {
        &self.packages
    }
    
    /// Get dependencies for a package
    pub fn get_dependencies(&self, package_key: &str) -> Option<&Vec<String>> {
        self.dependencies.get(package_key)
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Package manager for high-level package operations
#[derive(Debug)]
pub struct PackageManager {
    registry: PackageRegistry,
}

impl PackageManager {
    /// Create a new package manager
    pub fn new() -> Self {
        Self {
            registry: PackageRegistry::new(),
        }
    }
    
    /// Install packages from manifest
    pub fn install(&mut self, manifest: &PackageManifest) -> Result<()> {
        let dependency_graph = self.registry.resolve_dependencies(manifest)?;
        let install_order = dependency_graph.topological_order()?;
        
        for package_key in install_order {
            if let Some(package) = dependency_graph.packages().get(&package_key) {
                self.registry.install_package(&package.package.name, &package.package.version)?;
            }
        }
        
        Ok(())
    }
    
    /// Add a new dependency
    pub fn add_dependency(&mut self, name: &str, spec: DependencySpec) -> Result<()> {
        // TODO: Update manifest file with new dependency
        println!("Adding dependency: {} -> {:?}", name, spec);
        Ok(())
    }
    
    /// Remove a dependency
    pub fn remove_dependency(&mut self, name: &str) -> Result<()> {
        // TODO: Remove dependency from manifest and clean up
        println!("Removing dependency: {}", name);
        Ok(())
    }
    
    /// Update all dependencies
    pub fn update(&mut self) -> Result<()> {
        self.registry.update_cache()?;
        // TODO: Update all dependencies to latest compatible versions
        println!("Updating all dependencies...");
        Ok(())
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new()
    }
}
