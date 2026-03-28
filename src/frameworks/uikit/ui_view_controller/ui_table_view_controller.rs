/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableViewController`.

use crate::frameworks::foundation::{ns_array, NSUInteger};
use crate::objc::{
    autorelease, id, impl_HostObject_with_superclass, msg, nil, objc_classes, release, retain,
    ClassExports, NSZonePtr, SEL,
};

#[derive(Default)]
struct UITableViewControllerHostObject {
    superclass: super::UIViewControllerHostObject,
}
impl_HostObject_with_superclass!(UITableViewControllerHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableViewController: UIViewController

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITableViewControllerHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

@end

};
