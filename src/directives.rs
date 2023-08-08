/*!
 * Types of directives
 */

use crate::xact::Xact;


/// Types of directives
#[derive(Debug)]
 pub enum DirectiveType {
    Comment,
    Price,
    Xact(Xact)
}
