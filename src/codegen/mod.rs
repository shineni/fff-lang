
#![allow(unused_variables)]
// generate vm code from AST

// vm is a stack based virtual machine
// expand each expression-statement to vm codes, every expression-statement is independent
// other statements are all flow control statements, just add instructions include Goto, GotoIfTrue and GotoIfFalse
// expressions are expanded according to ExpressionOperators
//     special occassions, array.set_Index is combined from operator[] and operator=
// ExpressionOperators perform actions on evaluate stack, pop items and push items, 
// load local and store local interact between call stack and eval stack

// call stack and eval stack for functions are independent, so that rbp and rsp are not used
// vm will maintain the local variables with name as key
// a special vmcode called scope enter and scope leave is emitted at block enter and block leave, for block local variables

// In detail, expansion is 
// first, combine operator index and operator assignment, include op assignment to special set index and op set index
// only has last expr op and last expr op is function call or set index or op set index is allowed in expr statement
// then, make every expression in following ops to be pure base expression
// that is, move the operators and expr base to another statement and store the result in a generated local, and replace
//     previous expression by this temp local

mod vm_code;
mod session;
mod type_def;
mod fn_def;
mod var_def;
mod block;
mod statement;
mod expression;

pub use codegen::vm_code::InternalFn;
pub use codegen::vm_code::Operand;
pub use codegen::vm_code::Code;
pub use codegen::vm_code::CodeCollection;

use syntax::Program as SyntaxProgram;
pub fn generate(program: SyntaxProgram) -> CodeCollection {
    use codegen::session::GenerationSession;
    GenerationSession::dispatch(program)
}

// Designment2
// Input vector of syntax functions
// Prepare types collection with primitive types
//     primtive wide integral type, string, array and tuple, tuple is struct, array is a pointer, a heap object, size and cap are at heap not stack
// Prepare static var storage with empty, static var collection are at heap head when running
// Prepare fn collection with primitive functions
// Add primitive type other member functions to the fn collection
// Read in syntax function declares and store declares in fn collection store fn names in static var collection
// Process each fn implementation
// add fn args to this fn's var collection
//     Process each statement in the fn impl
//         Process each expression in some statement
//             every expr start from a literal or a var id or rax, and follow with several expr ops
//                 literal is integral literal or string literal, string literal is at static storage and not const, so can not intern
//                 for array literal include array dup literal, they are actually special function call, copy initialize values to heap
//                 for tuple literal, it is struct construction, copy initialize values to the stack
//             many kinds of ops
//                 for integral unary and binary ops, they are special vm codes and return to rax
//                 for other member function calls, they are function call
//                 for function calls, it is push args and jump and store rbp and rip and continue and return to the address of next var id
//                     if function return value is used, then the address of next var is used, other wise it is discarded
//                     so function call return value is pushed to the var collection but declared size zero
//                 for type cast, intergral primitive typecast is truncated or fill with zero or special integral to floating or floating to integral
//                     they return to rax
//                     no other type cast allowed currently
//                 Get index, or subscription, only applied to array, are just pointer alias

// Collections designment
// TypeCollection
//     new, new empty
//     add_prim_types, add primitive type decl
//     add_prim_types_impl, add primitive type member functions
//     get id, process SMType, may emit new
//     get_field_offset, get field offset
//     get_member_fn_id, get member function id or bin or un op in vmcode
// VarCollection
//     as current implementation, push var, push scope, pop scope, etc.
// FnCollection
//     new, new empty
//     add_prim_fns, add primitive global fns
//     add_prim_type_fns, add primtive type member fns
//     no check fn existence here, 
//         when meet global fn call, ask var collection and static var collection
//         when meet member fn call, ask type collection for the instance fn
//         when meet static member fn call, ask type collection for the static fn
// CodeCollection
//     emit
//     emit
//     emit
//     return CodeID
