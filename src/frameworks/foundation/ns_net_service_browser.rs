/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSNetServiceBrowser`.

use crate::objc::{
    autorelease, id, msg, nil, objc_classes, release, retain, ClassExports, HostObject, NSZonePtr,
};

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSNetServiceBrowser: NSObject

- (())setDelegate:(id)_delegate {
    // TODO
}

- (())searchForServicesOfType:(id)ofType // NSString
                     inDomain:(id)domainString {
    // TODO
}

@end

};
