//! A templating library like handle_bars or gtmpl (go templates) with a few extra features.
//!
//! * Closures as helper funtions.  (The reason I built this)
//! * Multiple parameters to templates and sub templates.
//!
//! This library primarily exists to support the "Siter" static website generator, which uses
//! templates for all content.
//!
//! ```rust
//! use templito::*;
//! use std::str::FromStr;
//! // get template somehow
//! let tp = r#"
//!     <h1>{{$0}}</h1>
//!     {{for k v in $1}}\
//!         <p>{{$k}} = {{$v}}</p>
//!     {{/for}}\
//!     {{#Get a list member by reference#}}\
//!     {{$1.1}}
//!     {{#Or Even Markdown on a templatable block#}}\
//!     {{@md}}
//! * {{$1.0}}
//! * {{$1.2}}
//!     {{/md}}
//! "#;
//!
//! // The function manager can be build separately to a the FuncMan trait
//! let fm = func_man::default_func_man();
//!
//! // The Template manager will normally search for and load templates
//! // on request. Which is why it must be mutable.
//! // NoTemplates will always return an Err.
//! let mut tm = temp_man::NoTemplates;
//!
//! //Vec<&str> implements TParam so it can be sent as a parameter
//! let animals = vec!["cat","dog","fish"];
//!
//! //so does &str
//! let data:Vec<&dyn TParam> = vec![&"hello", &animals];
//!
//! let tp = TreeTemplate::from_str(tp).unwrap();
//!
//! let s = tp.run(&data,&mut tm,&fm).unwrap();
//! assert_eq!(s,r#"
//!     <h1>hello</h1>
//!     <p>0 = cat</p>
//!     <p>1 = dog</p>
//!     <p>2 = fish</p>
//!     dog
//!     <ul>
//! <li>cat</li>
//! <li>fish</li>
//! </ul>
//!
//! "#);
//!
//! ```
//! The Default Func manager provides functions for strings, maths, bools and paths.
//!
//! But new functions are relatively easy to add. once you understand how the underlying
//! structures work.
//!
//! "TData" Is the main enum that holds the possible types for passing around, including
//! String, Bool, Int, Uint,Float,Map<String,TData>,List<TData>
//!
//! "TCow" Is a Borrow/Concrete type allowing types that can return a borrow to do so.
//! While those that must create their responce to do that too.  This means big lists and complex
//! types can avoid being copied in many places.
//!
//! Helper Functions have to follow one of the following signatures:
//!
//! ```ignore
//! type TFunc = dyn Fn(&[TCow<'a>]) -> Result<TCow<'a>>;
//! type TFn = fn'a(_: &[TCow<'a>]) -> Result<TCow<'a>>;
//!
//! ```
//! and can be added to BasicFuncs using
//!
//!
//!

pub mod func_man;
pub mod funcs;
pub mod parse;
//mod pipeline;
mod scope;
pub mod tdata;
pub mod temp_man;
pub mod template;
mod tests;
pub mod tparam;
pub use template::TreeTemplate;
pub mod prelude;
pub use funcs::WithFuncs;
pub use tdata::TData;
pub use tparam::TParam;
pub mod expr;
mod pattern;
