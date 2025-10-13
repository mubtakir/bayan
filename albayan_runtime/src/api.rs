//! C API for LLVM integration
//! Expert recommendation: Priority 2 - Runtime API functions

use crate::knowledge_base::{KnowledgeBase, Value, Term};
use crate::solver::{LogicSolver, SolutionIterator, Solution};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;

/// Global knowledge base (Expert recommendation: Runtime state)
static mut GLOBAL_KB: Option<Mutex<KnowledgeBase>> = None;

/// Global solution iterators (Expert recommendation: Priority 1 - query_solve support)
static mut GLOBAL_ITERATORS: Option<Mutex<HashMap<u64, SolutionIterator<'static>>>> = None;

/// Global solutions (Expert recommendation: Priority 1 - solution handles)
static mut GLOBAL_SOLUTIONS: Option<Mutex<HashMap<u64, Solution>>> = None;

/// Next handle ID (Expert recommendation: Priority 1)
static mut NEXT_HANDLE_ID: u64 = 1;

/// Initialize the runtime (Expert recommendation: Called from LLVM generated code)
#[no_mangle]
pub extern "C" fn albayan_rt_init() {
    unsafe {
        GLOBAL_KB = Some(Mutex::new(KnowledgeBase::new()));
        GLOBAL_ITERATORS = Some(Mutex::new(HashMap::new()));
        GLOBAL_SOLUTIONS = Some(Mutex::new(HashMap::new()));
        NEXT_HANDLE_ID = 1;
    }
}

/// Cleanup the runtime (Expert recommendation: Called at program exit)
#[no_mangle]
pub extern "C" fn albayan_rt_cleanup() {
    unsafe {
        GLOBAL_KB = None;
        GLOBAL_ITERATORS = None;
        GLOBAL_SOLUTIONS = None;
        NEXT_HANDLE_ID = 1;
    }
}

/// Register a relation (Expert recommendation: Called for relation declarations)
#[no_mangle]
pub extern "C" fn albayan_rt_register_relation(
    name: *const c_char,
    arity: c_int,
    arg_types: *const *const c_char,
) -> c_int {
    if name.is_null() || arg_types.is_null() {
        return -1; // Error
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    let mut type_names = Vec::new();
    for i in 0..arity {
        let type_ptr = unsafe { *arg_types.offset(i as isize) };
        if type_ptr.is_null() {
            return -1;
        }
        let type_str = unsafe {
            match CStr::from_ptr(type_ptr).to_str() {
                Ok(s) => s.to_string(),
                Err(_) => return -1,
            }
        };
        type_names.push(type_str);
    }

    unsafe {
        if let Some(ref kb_mutex) = GLOBAL_KB {
            if let Ok(mut kb) = kb_mutex.lock() {
                kb.register_relation(name_str, arity as usize, type_names);
                return 0; // Success
            }
        }
    }

    -1 // Error
}

/// Assert a fact (Expert recommendation: Called for fact statements)
#[no_mangle]
pub extern "C" fn albayan_rt_assert_fact(
    relation: *const c_char,
    args: *const *const c_char,
    arg_types: *const *const c_char,
    arity: c_int,
) -> c_int {
    if relation.is_null() || args.is_null() || arg_types.is_null() {
        return -1;
    }

    let relation_str = unsafe {
        match CStr::from_ptr(relation).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    let mut values = Vec::new();
    for i in 0..arity {
        let arg_ptr = unsafe { *args.offset(i as isize) };
        let type_ptr = unsafe { *arg_types.offset(i as isize) };

        if arg_ptr.is_null() || type_ptr.is_null() {
            return -1;
        }

        let arg_str = unsafe {
            match CStr::from_ptr(arg_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return -1,
            }
        };

        let type_str = unsafe {
            match CStr::from_ptr(type_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return -1,
            }
        };

        match Value::from_string(arg_str, type_str) {
            Ok(value) => values.push(value),
            Err(_) => return -1,
        }
    }

    unsafe {
        if let Some(ref kb_mutex) = GLOBAL_KB {
            if let Ok(mut kb) = kb_mutex.lock() {
                match kb.assert_fact(relation_str, values) {
                    Ok(_) => return 0,
                    Err(_) => return -1,
                }
            }
        }
    }

    -1
}

/// Register a rule (Expert recommendation: Called for rule statements)
#[no_mangle]
pub extern "C" fn albayan_rt_register_rule(
    head_relation: *const c_char,
    head_args: *const *const c_char,
    head_arity: c_int,
    body_relations: *const *const c_char,
    body_args: *const *const *const c_char,
    body_arities: *const c_int,
    body_count: c_int,
) -> c_int {
    // Simplified implementation for now
    // In a full implementation, this would parse the serialized rule structure

    if head_relation.is_null() {
        return -1;
    }

    // For now, just return success
    // TODO: Implement full rule parsing and registration
    0
}

/// Query/prove a goal (Expert recommendation: Called for query_prove statements)
#[no_mangle]
pub extern "C" fn albayan_rt_query_prove(
    relation: *const c_char,
    args: *const *const c_char,
    arg_types: *const *const c_char,
    arity: c_int,
    result_buffer: *mut c_char,
    buffer_size: c_int,
) -> c_int {
    if relation.is_null() || args.is_null() || arg_types.is_null() || result_buffer.is_null() {
        return -1;
    }

    let relation_str = unsafe {
        match CStr::from_ptr(relation).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -1,
        }
    };

    // Build query term
    let mut query_args = Vec::new();
    for i in 0..arity {
        let arg_ptr = unsafe { *args.offset(i as isize) };
        let type_ptr = unsafe { *arg_types.offset(i as isize) };

        if arg_ptr.is_null() || type_ptr.is_null() {
            return -1;
        }

        let arg_str = unsafe {
            match CStr::from_ptr(arg_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return -1,
            }
        };

        let type_str = unsafe {
            match CStr::from_ptr(type_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return -1,
            }
        };

        // Check if this is a variable (starts with ?)
        if arg_str.starts_with('?') {
            query_args.push(Term::variable(&arg_str[1..])); // Remove ? prefix
        } else {
            match Value::from_string(arg_str, type_str) {
                Ok(value) => query_args.push(Term::constant(value)),
                Err(_) => return -1,
            }
        }
    }

    let query = Term::compound(relation_str, query_args);

    unsafe {
        if let Some(ref kb_mutex) = GLOBAL_KB {
            if let Ok(kb) = kb_mutex.lock() {
                let solver = LogicSolver::new(&kb);
                let result = solver.prove(&query);

                // Format result as JSON string
                let result_json = format!(
                    "{{\"success\":{},\"solutions\":{}}}",
                    result.success,
                    result.bindings.len()
                );

                // Copy to result buffer
                let result_cstring = match CString::new(result_json) {
                    Ok(s) => s,
                    Err(_) => return -1,
                };

                let result_bytes = result_cstring.as_bytes_with_nul();
                if result_bytes.len() > buffer_size as usize {
                    return -1; // Buffer too small
                }

                std::ptr::copy_nonoverlapping(
                    result_bytes.as_ptr(),
                    result_buffer as *mut u8,
                    result_bytes.len(),
                );

                return if result.success { 1 } else { 0 };
            }
        }
    }

    -1
}

/// Get the number of solutions for a query (Expert recommendation: Utility function)
#[no_mangle]
pub extern "C" fn albayan_rt_count_solutions(
    relation: *const c_char,
    args: *const *const c_char,
    arg_types: *const *const c_char,
    arity: c_int,
) -> c_int {
    // Similar to query_prove but just return count
    // Implementation would be similar to above
    -1 // Placeholder
}

/// Check if a query can be proven (Expert recommendation: Boolean query)
#[no_mangle]
pub extern "C" fn albayan_rt_can_prove(
    relation: *const c_char,
    args: *const *const c_char,
    arg_types: *const *const c_char,
    arity: c_int,
) -> c_int {
    // Similar to query_prove but just return boolean result
    // Implementation would be similar to above
    0 // Placeholder
}

/// Create a solution iterator for query_solve (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_query_solve(
    relation: *const c_char,
    args: *const *const c_char,
    arg_types: *const *const c_char,
    arity: c_int,
) -> u64 {
    if relation.is_null() || args.is_null() || arg_types.is_null() {
        return 0; // Invalid handle
    }

    let relation_str = unsafe {
        match CStr::from_ptr(relation).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 0,
        }
    };

    // Build query term (similar to query_prove)
    let mut query_args = Vec::new();
    for i in 0..arity {
        let arg_ptr = unsafe { *args.offset(i as isize) };
        let type_ptr = unsafe { *arg_types.offset(i as isize) };

        if arg_ptr.is_null() || type_ptr.is_null() {
            return 0;
        }

        let arg_str = unsafe {
            match CStr::from_ptr(arg_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return 0,
            }
        };

        let type_str = unsafe {
            match CStr::from_ptr(type_ptr).to_str() {
                Ok(s) => s,
                Err(_) => return 0,
            }
        };

        // Check if this is a variable (starts with ?)
        if arg_str.starts_with('?') {
            query_args.push(Term::variable(&arg_str[1..])); // Remove ? prefix
        } else {
            match Value::from_string(arg_str, type_str) {
                Ok(value) => query_args.push(Term::constant(value)),
                Err(_) => return 0,
            }
        }
    }

    let query = Term::compound(relation_str, query_args);

    unsafe {
        if let Some(ref kb_mutex) = GLOBAL_KB {
            if let Ok(kb) = kb_mutex.lock() {
                let solver = LogicSolver::new(&kb);

                // Create solution iterator
                let iterator = solver.solve_iter(&query);

                // Store iterator and return handle
                if let Some(ref iterators_mutex) = GLOBAL_ITERATORS {
                    if let Ok(mut iterators) = iterators_mutex.lock() {
                        let handle = NEXT_HANDLE_ID;
                        NEXT_HANDLE_ID += 1;

                        // Note: This is a simplified approach. In a real implementation,
                        // we'd need to handle lifetimes more carefully
                        // For now, we'll store the iterator with a static lifetime
                        let static_iterator: SolutionIterator<'static> = std::mem::transmute(iterator);
                        iterators.insert(handle, static_iterator);

                        return handle;
                    }
                }
            }
        }
    }

    0 // Error
}

/// Get the next solution from an iterator (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_iterator_next(iterator_handle: u64) -> u64 {
    if iterator_handle == 0 {
        return 0; // Invalid handle
    }

    unsafe {
        if let Some(ref iterators_mutex) = GLOBAL_ITERATORS {
            if let Ok(mut iterators) = iterators_mutex.lock() {
                if let Some(iterator) = iterators.get_mut(&iterator_handle) {
                    if let Some(solution) = iterator.next_solution() {
                        // Store solution and return handle
                        if let Some(ref solutions_mutex) = GLOBAL_SOLUTIONS {
                            if let Ok(mut solutions) = solutions_mutex.lock() {
                                let handle = NEXT_HANDLE_ID;
                                NEXT_HANDLE_ID += 1;

                                solutions.insert(handle, solution);
                                return handle;
                            }
                        }
                    }
                }
            }
        }
    }

    0 // No more solutions or error
}

/// Get variable value from a solution (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_solution_get_var(
    solution_handle: u64,
    var_name: *const c_char,
    result_buffer: *mut c_char,
    buffer_size: c_int,
) -> c_int {
    if solution_handle == 0 || var_name.is_null() || result_buffer.is_null() {
        return -1; // Error
    }

    let var_name_str = unsafe {
        match CStr::from_ptr(var_name).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        }
    };

    unsafe {
        if let Some(ref solutions_mutex) = GLOBAL_SOLUTIONS {
            if let Ok(solutions) = solutions_mutex.lock() {
                if let Some(solution) = solutions.get(&solution_handle) {
                    if let Some(term) = solution.get_variable_value(var_name_str) {
                        // Convert term to string representation
                        let term_str = format!("{:?}", term); // Simplified serialization

                        let term_cstring = match CString::new(term_str) {
                            Ok(s) => s,
                            Err(_) => return -1,
                        };

                        let term_bytes = term_cstring.as_bytes_with_nul();
                        if term_bytes.len() > buffer_size as usize {
                            return -1; // Buffer too small
                        }

                        std::ptr::copy_nonoverlapping(
                            term_bytes.as_ptr(),
                            result_buffer as *mut u8,
                            term_bytes.len(),
                        );

                        return 0; // Success
                    }
                }
            }
        }
    }

    -1 // Variable not found or error
}

/// Check if iterator has more solutions (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_iterator_has_more(iterator_handle: u64) -> c_int {
    if iterator_handle == 0 {
        return 0; // Invalid handle
    }

    unsafe {
        if let Some(ref iterators_mutex) = GLOBAL_ITERATORS {
            if let Ok(iterators) = iterators_mutex.lock() {
                if let Some(iterator) = iterators.get(&iterator_handle) {
                    return if iterator.has_more() { 1 } else { 0 };
                }
            }
        }
    }

    0 // No more solutions or error
}

/// Cleanup iterator (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_iterator_cleanup(iterator_handle: u64) -> c_int {
    if iterator_handle == 0 {
        return -1; // Invalid handle
    }

    unsafe {
        if let Some(ref iterators_mutex) = GLOBAL_ITERATORS {
            if let Ok(mut iterators) = iterators_mutex.lock() {
                iterators.remove(&iterator_handle);
                return 0; // Success
            }
        }
    }

    -1 // Error
}

/// Cleanup solution (Expert recommendation: Priority 1)
#[no_mangle]
pub extern "C" fn albayan_rt_solution_cleanup(solution_handle: u64) -> c_int {
    if solution_handle == 0 {
        return -1; // Invalid handle
    }

    unsafe {
        if let Some(ref solutions_mutex) = GLOBAL_SOLUTIONS {
            if let Ok(mut solutions) = solutions_mutex.lock() {
                solutions.remove(&solution_handle);
                return 0; // Success
            }
        }
    }

    -1 // Error
}
