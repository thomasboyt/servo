/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Traits that nodes must implement. Breaks the otherwise-cyclic dependency between layout and
//! style.

use selectors::AttrSelector;
use servo_util::namespace::Namespace;


pub trait TNode<E:TElement> : Clone {
    fn parent_node(&self) -> Option<Self>;
    fn prev_sibling(&self) -> Option<Self>;
    fn next_sibling(&self) -> Option<Self>;
    fn is_document(&self) -> bool;
    fn is_element(&self) -> bool;

    /// FIXME(pcwalton): This should not use the `with` pattern.
    fn with_element<'a, R>(&self, f: |&E| -> R) -> R;

    fn match_attr(&self, attr: &AttrSelector, test: |&str| -> bool) -> bool;
}

pub trait TElement {
    fn get_attr(&self, namespace: &Namespace, attr: &str) -> Option<&'static str>;
    fn get_link(&self) -> Option<&'static str>;
    fn get_local_name<'a>(&'a self) -> &'a str;
    fn get_namespace<'a>(&'a self) -> &'a Namespace;
    fn get_hover_state(&self) -> bool;
}

