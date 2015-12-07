//! Provides backend-agnostic BLAS operations for [Collenchyma][collenchyma].
//!
//! BLAS (Basic Linear Algebra Subprograms) is a specification that prescribes a set of low-level
//! routines for performing common linear algebra operations such as vector addition, scalar
//! multiplication, dot products, linear combinations, and matrix multiplication. They are the de
//! facto standard low-level routines for linear algebra libraries; the routines have bindings for
//! both C and Fortran. Although the BLAS specification is general, BLAS implementations are often
//! optimized for speed on a particular machine, so using them can bring substantial performance
//! benefits. BLAS implementations will take advantage of special floating point hardware such as
//! vector registers or SIMD instructions.<br/>
//! [Source][blas-source]
//!
//! ## Overview
//!
//! A Collenchyma library describes the functionality through three types of traits.
//!
//! * __PluginTrait__ -> IBlas<br/>
//! This trait provides 'provided methods', which already provide the exact, backend-agnostic
//! behavior of a library Operation. These come in two forms `operation()` and `operation_plain()`,
//! where the first takes care of full memory management and the later one just provides the computation
//! without any memory management. In some scenarios you would like to use the plain operation for faster
//! exection.
//!
//! * __BinaryTrait__ -> IBlasBinary<br>
//! The binary trait provides the actual and potentially initialized Functions, which are able to compute
//! the operations (as they implement the OperationTrait).
//!
//! * __OperationTrait__ -> e.g. IOperationDot<br/>
//! The PluginTrait can provide 'provided methods', thanks to the OperationTrait. The OperationTrait,
//! has one required method `compute` which every Framework Function will implement in it's own way.
//!
//! Beside these traits a Collenchyma Plugin might also use macros for faster
//! implementation for various Collenchyma Frameworks such as CUDA, OpenCL or common host CPU.
//!
//! For more information, give the [Collenchyma docs][collenchyma-docs] a visit.
//!
//! [collenchyma]: https://github.com/autumnai/collenchyma
//! [collenchyma-docs]: http://autumnai.github.io/collenchyma
//! [blas-source]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
#![cfg_attr(lint, feature(plugin))]
#![cfg_attr(lint, plugin(clippy))]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_import_braces, unused_qualifications)]

extern crate collenchyma;
extern crate rblas as blas;

pub mod library;
pub mod binary;
pub mod operation;
#[macro_use]
pub mod helper;
mod frameworks;