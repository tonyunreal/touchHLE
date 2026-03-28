/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `MPVolumeView`.

use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize};
use crate::frameworks::foundation::NSInteger;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, nil, objc_classes, todo_objc_setter, ClassExports,
    NSZonePtr, SEL,
};

pub struct MPVolumeViewHostObject {
    superclass: super::UIViewHostObject,

}
impl_HostObject_with_superclass!(MPVolumeViewHostObject);
impl Default for MPVolumeViewHostObject {
    fn default() -> Self {
        MPVolumeViewHostObject {
            superclass: Default::default(),
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation MPVolumeView: UIView

@end

};
