//! Implements a number stepper. By default this uses NSStepper on macOS.

use core_graphics::geometry::CGRect;
use objc::rc::{Id, Shared};
use objc::runtime::{Class, Object};
use objc::{msg_send, msg_send_id, sel};

use crate::control::Control;
use crate::foundation::{id, load_or_register_class, nil, NSInteger, NSNumber, NSString, NO, YES};
use crate::geometry::Rect;
use crate::invoker::TargetActionHandler;
use crate::layout::Layout;
#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;

/// Wraps `NSStepper` on AppKit. Not currently implemented for iOS.
#[derive(Debug)]
pub struct Stepper {
    /// A handle for the underlying Objective-C object.
    pub objc: ObjcProperty,

    handler: Option<TargetActionHandler>,

    /// A pointer to the Objective-C runtime top layout constraint.
    #[cfg(feature = "autolayout")]
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    #[cfg(feature = "autolayout")]
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    #[cfg(feature = "autolayout")]
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    #[cfg(feature = "autolayout")]
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    #[cfg(feature = "autolayout")]
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    #[cfg(feature = "autolayout")]
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    #[cfg(feature = "autolayout")]
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    #[cfg(feature = "autolayout")]
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_y: LayoutAnchorY,
}

impl Stepper {
    /// Creates a new `Stepper` instance, configures it appropriately,
    /// and retains the necessary Objective-C runtime pointer.
    pub fn new() -> Self {
        let zero: CGRect = Rect::zero().into();

        let view: id = unsafe {
            let alloc: id = msg_send![register_class(), alloc];
            let stepper: id = msg_send![alloc, initWithFrame:zero];

            #[cfg(feature = "autolayout")]
            let _: () = msg_send![stepper, setTranslatesAutoresizingMaskIntoConstraints: NO];

            stepper
        };

        Stepper {
            handler: None,

            #[cfg(feature = "autolayout")]
            top: LayoutAnchorY::top(view),

            #[cfg(feature = "autolayout")]
            left: LayoutAnchorX::left(view),

            #[cfg(feature = "autolayout")]
            leading: LayoutAnchorX::leading(view),

            #[cfg(feature = "autolayout")]
            right: LayoutAnchorX::right(view),

            #[cfg(feature = "autolayout")]
            trailing: LayoutAnchorX::trailing(view),

            #[cfg(feature = "autolayout")]
            bottom: LayoutAnchorY::bottom(view),

            #[cfg(feature = "autolayout")]
            width: LayoutAnchorDimension::width(view),

            #[cfg(feature = "autolayout")]
            height: LayoutAnchorDimension::height(view),

            #[cfg(feature = "autolayout")]
            center_x: LayoutAnchorX::center(view),

            #[cfg(feature = "autolayout")]
            center_y: LayoutAnchorY::center(view),

            objc: ObjcProperty::retain(view),
        }
    }

    /// Attaches a callback
    pub fn set_action<F: Fn(*const Object) + Send + Sync + 'static>(&mut self, action: F) {
        // @TODO: This probably isn't ideal but gets the job done for now; needs revisiting.
        let this: Id<Object, Shared> = self.objc.get(|obj| unsafe { msg_send_id![obj, self] });
        let handler = TargetActionHandler::new(&this, action);
        self.handler = Some(handler);
    }

    /// Sets maximum value
    pub fn set_max_value(&self, value: f64) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setMaxValue: value];
        });
    }

    /// Sets minimum value
    pub fn set_min_value(&self, value: f64) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setMinValue: value];
        });
    }

    /// Sets increment
    pub fn set_increment(&self, value: f64) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setIncrement: value];
        });
    }

    /// Sets whether this wraps
    pub fn set_wraps(&self, wraps: bool) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setValueWraps:match wraps {
                true => YES,
                false => NO
            }];
        });
    }

    /// Gets the selected index.
    pub fn get_selected_value(&self) -> f32 {
        self.objc.get(|obj| unsafe { msg_send![obj, floatValue] })
    }
}

impl ObjcAccess for Stepper {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for Stepper {
    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(
            r#"
            Tried to add a subview to a Stepper. This is not allowed in Cacao. If you think this should be supported,
            open a discussion on the GitHub repo.
        "#
        );
    }
}

impl Control for Stepper {}

impl ObjcAccess for &Stepper {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for &Stepper {
    fn add_subview<V: Layout>(&self, _view: &V) {
        panic!(
            r#"
            Tried to add a subview to a Stepper. This is not allowed in Cacao. If you think this should be supported,
            open a discussion on the GitHub repo.
        "#
        );
    }
}

impl Control for &Stepper {}

impl Drop for Stepper {
    /// Nils out references on the Objective-C side and removes this from the backing view.
    // Just to be sure, let's... nil these out. They should be weak references,
    // but I'd rather be paranoid and remove them later.
    fn drop(&mut self) {
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![obj, setTarget: nil];
            let _: () = msg_send![obj, setAction: nil];
        });
    }
}

/// Registers an `NSStepper` subclass, and configures it to hold some ivars
/// for various things we need to store.
fn register_class() -> &'static Class {
    load_or_register_class("NSStepper", "CacaoStepper", |decl| unsafe {})
}
