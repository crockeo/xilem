// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use accesskit::Role;
use smallvec::{smallvec, SmallVec};
use tracing::{trace_span, Span};
use vello::kurbo::Point;
use vello::Scene;

use crate::widget::{WidgetMut, WidgetPod};
use crate::{
    AccessCtx, AccessEvent, BoxConstraints, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    PointerEvent, Size, StatusChange, TextEvent, Widget, WidgetId,
};

// TODO: This is a hack to provide an accessibility node with a Window type.
// This should eventually be removed.
pub struct RootWidget<W> {
    pub(crate) pod: WidgetPod<W>,
}

impl<W: Widget> RootWidget<W> {
    pub fn new(widget: W) -> RootWidget<W> {
        RootWidget {
            pod: WidgetPod::new(widget),
        }
    }

    // TODO - This help works around impedance mismatch between the types of Xilem and Masonry
    pub fn from_pod(pod: WidgetPod<W>) -> RootWidget<W> {
        RootWidget { pod }
    }
}

impl<W: Widget> WidgetMut<'_, RootWidget<W>> {
    // TODO - rename to child_mut
    pub fn get_element(&mut self) -> WidgetMut<'_, W> {
        self.ctx.get_mut(&mut self.widget.pod)
    }
}

impl<W: Widget> Widget for RootWidget<W> {
    fn on_pointer_event(&mut self, _ctx: &mut EventCtx, _event: &PointerEvent) {}
    fn on_text_event(&mut self, _ctx: &mut EventCtx, _event: &TextEvent) {}
    fn on_access_event(&mut self, _ctx: &mut EventCtx, _event: &AccessEvent) {}

    fn on_status_change(&mut self, _: &mut LifeCycleCtx, _: &StatusChange) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle) {
        self.pod.lifecycle(ctx, event);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints) -> Size {
        let size = self.pod.layout(ctx, bc);
        ctx.place_child(&mut self.pod, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, scene: &mut Scene) {
        self.pod.paint(ctx, scene);
    }

    fn accessibility_role(&self) -> Role {
        Role::Window
    }

    fn accessibility(&mut self, ctx: &mut AccessCtx) {
        self.pod.accessibility(ctx);
    }

    fn children_ids(&self) -> SmallVec<[WidgetId; 16]> {
        smallvec![self.pod.id()]
    }

    fn make_trace_span(&self) -> Span {
        trace_span!("RootWidget")
    }
}
