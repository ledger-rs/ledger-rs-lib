/*!
 * Types of directives
 */

use crate::viewmodel::Xact;

/// Types of directives
#[derive(Debug)]
 pub enum DirectiveType {
    Comment,
    Price,
    Xact(Xact)
}
