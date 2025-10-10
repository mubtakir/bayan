//! # Memory Management
//! 
//! This module implements memory management for the AlBayan runtime.

use std::collections::HashMap;
use std::alloc::{Layout, alloc, dealloc};
use super::RuntimeError;

/// Memory manager for the AlBayan runtime
#[derive(Debug)]
pub struct MemoryManager {
    /// Maximum memory that can be allocated
    max_memory: usize,
    
    /// Currently allocated memory
    allocated: usize,
    
    /// Peak memory usage
    peak_allocated: usize,
    
    /// Active allocations (ptr -> size)
    allocations: HashMap<*mut u8, usize>,
    
    /// Garbage collection enabled
    gc_enabled: bool,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new(max_memory: usize) -> Self {
        Self {
            max_memory,
            allocated: 0,
            peak_allocated: 0,
            allocations: HashMap::new(),
            gc_enabled: true,
        }
    }
    
    /// Allocate memory
    pub fn allocate(&mut self, size: usize) -> Result<*mut u8, RuntimeError> {
        if self.allocated + size > self.max_memory {
            // Try garbage collection first
            if self.gc_enabled {
                self.garbage_collect()?;
            }
            
            // Check again after GC
            if self.allocated + size > self.max_memory {
                return Err(RuntimeError::MemoryError(
                    format!("Out of memory: requested {}, available {}", 
                           size, self.max_memory - self.allocated)
                ));
            }
        }
        
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| RuntimeError::MemoryError(e.to_string()))?;
        
        let ptr = unsafe { alloc(layout) };
        
        if ptr.is_null() {
            return Err(RuntimeError::MemoryError("System allocation failed".to_string()));
        }
        
        self.allocated += size;
        if self.allocated > self.peak_allocated {
            self.peak_allocated = self.allocated;
        }
        
        self.allocations.insert(ptr, size);
        
        Ok(ptr)
    }
    
    /// Deallocate memory
    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) -> Result<(), RuntimeError> {
        if ptr.is_null() {
            return Ok(());
        }
        
        if let Some(&allocated_size) = self.allocations.get(&ptr) {
            if allocated_size != size {
                return Err(RuntimeError::MemoryError(
                    format!("Size mismatch: expected {}, got {}", allocated_size, size)
                ));
            }
            
            let layout = Layout::from_size_align(size, 8)
                .map_err(|e| RuntimeError::MemoryError(e.to_string()))?;
            
            unsafe { dealloc(ptr, layout) };
            
            self.allocated -= size;
            self.allocations.remove(&ptr);
            
            Ok(())
        } else {
            Err(RuntimeError::MemoryError("Attempting to free unallocated memory".to_string()))
        }
    }
    
    /// Run garbage collection
    pub fn garbage_collect(&mut self) -> Result<(), RuntimeError> {
        // Simple mark-and-sweep garbage collection
        // In a real implementation, this would trace reachable objects
        
        // For now, just report that GC ran
        println!("Garbage collection completed");
        Ok(())
    }
    
    /// Clean up all allocations
    pub fn cleanup(&mut self) -> Result<(), RuntimeError> {
        let allocations: Vec<(*mut u8, usize)> = self.allocations.iter()
            .map(|(&ptr, &size)| (ptr, size))
            .collect();
        
        for (ptr, size) in allocations {
            self.deallocate(ptr, size)?;
        }
        
        Ok(())
    }
    
    /// Get total allocated memory
    pub fn total_allocated(&self) -> usize {
        self.allocated
    }
    
    /// Get peak allocated memory
    pub fn peak_allocated(&self) -> usize {
        self.peak_allocated
    }
    
    /// Get number of active allocations
    pub fn allocation_count(&self) -> usize {
        self.allocations.len()
    }
    
    /// Enable or disable garbage collection
    pub fn set_gc_enabled(&mut self, enabled: bool) {
        self.gc_enabled = enabled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let mut manager = MemoryManager::new(1024);
        
        let ptr = manager.allocate(64).unwrap();
        assert!(!ptr.is_null());
        assert_eq!(manager.total_allocated(), 64);
        
        manager.deallocate(ptr, 64).unwrap();
        assert_eq!(manager.total_allocated(), 0);
    }
    
    #[test]
    fn test_memory_limit() {
        let mut manager = MemoryManager::new(100);
        
        // This should fail
        let result = manager.allocate(200);
        assert!(result.is_err());
    }
}
