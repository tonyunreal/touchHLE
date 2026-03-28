/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `UITableView`.

use crate::frameworks::core_graphics::{CGPoint, CGRect, CGSize, CGFloat};
use crate::frameworks::foundation::NSInteger;
use crate::frameworks::uikit::ui_view::ui_scroll_view::UIScrollViewHostObject;
use crate::objc::{
    id, impl_HostObject_with_superclass, msg, nil, objc_classes, todo_objc_setter, ClassExports,
    NSZonePtr, SEL,
};
use crate::{msg_super};

pub type UITableViewStyle = NSInteger;
#[allow(unused)]
pub const UITableViewStylePlain: UITableViewStyle = 0;
#[allow(unused)]
pub const UITableViewStyleGrouped: UITableViewStyle = 1 << 0;

pub struct UITableViewHostObject {
    superclass: UIScrollViewHostObject,
    /// UITableViewDelegate, weak reference
    delegate: id,
    dataSource: id,
}
impl_HostObject_with_superclass!(UITableViewHostObject);
impl Default for UITableViewHostObject {
    fn default() -> Self {
        UITableViewHostObject {
            superclass: Default::default(),
            delegate: nil,
            dataSource: nil,
        }
    }
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UITableView: UIScrollView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<UITableViewHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (id)initWithFrame:(CGRect)frame style:(UITableViewStyle)style {
    let this: id = msg_super![env; this initWithFrame:frame];
    this
}

- (id)delegate {
    env.objc.borrow::<UITableViewHostObject>(this).delegate
}
- (())setDelegate:(id)delegate {
    env.objc.borrow_mut::<UITableViewHostObject>(this).delegate = delegate;
}

- (id)dataSource {
    env.objc.borrow::<UITableViewHostObject>(this).dataSource
}
- (())setDataSource:(id)dataSource {
    env.objc.borrow_mut::<UITableViewHostObject>(this).dataSource = dataSource;
}

- (())setRowHeight:(CGFloat)height {

}

- (())setShowsVerticalScrollIndicator:(bool)showIndicator {

}

- (())setShowsHorizontalScrollIndicator:(bool)showIndicator {
    
}

@end

@implementation UITableViewCell: UIView

@end

};
