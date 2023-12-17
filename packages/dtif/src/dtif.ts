import type * as bindings from './gen/bindings';

export type TAbsoluteTransformMixin = bindings.AbsoluteTransformMixin;
export type TAnchor = bindings.Anchor;
export type TAnchorCommand = bindings.AnchorCommand;
export type TAnyCoreInputEvent = bindings.AnyCoreInputEvent;
export type TAnyInputEvent = bindings.AnyInputEvent;
export type TAnyInteractionInputEvent = bindings.AnyInteractionInputEvent;
export type TAttributeRemoved = bindings.AttributeRemoved;
export type TAttributeUpdated = bindings.AttributeUpdated;
export type TBlendMixin = bindings.BlendMixin;
export type TBlendMode = bindings.BlendMode;
export type TBreakLineOn = bindings.BreakLineOn;
export type TChildrenMixin = bindings.ChildrenMixin;
export type TCoreInputEvent = bindings.CoreInputEvent;
export type TCursorChangeEvent = bindings.CursorChangeEvent;
export type TCursorDownOnComposition = bindings.CursorDownOnComposition;
export type TCursorDownOnEntity = bindings.CursorDownOnEntity;
export type TCursorDownOnResizeHandle = bindings.CursorDownOnResizeHandle;
export type TCursorDownOnRotateHandle = bindings.CursorDownOnRotateHandle;
export type TCursorEnteredComposition = bindings.CursorEnteredComposition;
export type TCursorExitedComposition = bindings.CursorExitedComposition;
export type TCursorForFrontend = bindings.CursorForFrontend;
export type TCursorMovedOnComposition = bindings.CursorMovedOnComposition;
export type TCursorUpOnComposition = bindings.CursorUpOnComposition;
export type TComposition = Omit<bindings.DTIFComposition, 'fonts'> & {
	fonts: Record<string, TFontWithContent>;
};
export type TNode = bindings.DTIFNode;
export type TDimensionMixin = bindings.DimensionMixin;
export type TElementAppended = bindings.ElementAppended;
export type TElementCreated = bindings.ElementCreated;
export type TElementDeleted = bindings.ElementDeleted;
export type TEllipse = bindings.Ellipse;
export type TEllipseArcData = bindings.EllipseArcData;
export type TEntity = bindings.Entity;
export type TEntityMoved = bindings.EntityMoved;
export type TEntitySetPosition = bindings.EntitySetPosition;
export type TFillMixin = bindings.FillMixin;
export type TFontMetadata = bindings.FontMetadata;
export type TFontStyle = bindings.FontStyle;
export type TFontWithContent = Omit<bindings.FontWithContent, 'content'> & {
	/**
	 * The actual content of the font as binary data
	 * or a URL string pointing to the file location.
	 *
	 * Note: FontWithContent doesn't support URL as its not that easy making a request from WASM
	 * https://stackoverflow.com/questions/66397775/how-do-i-make-an-http-request-within-a-wasm-bindgen-function-in-rust
	 */
	content: number[] | string;
};
export type TFrame = bindings.Frame;
export type TFrameNodeBundle = bindings.FrameNodeBundle;
export type TGroup = bindings.Group;
export type TGroupNodeBundle = bindings.GroupNodeBundle;
export type THandleSide = bindings.HandleSide;
export type THorizontalTextAlignment = bindings.HorizontalTextAlignment;
export type TInteractionInputEvent = bindings.InteractionInputEvent;
export type TInteractionModeChangeEvent = bindings.InteractionModeChangeEvent;
export type TInteractionModeForFrontend = bindings.InteractionModeForFrontend;
export type TLetterSpacing = bindings.LetterSpacing;
export type TLineHeight = bindings.LineHeight;
export type TLocked = bindings.Locked;
export type TMat3 = bindings.Mat3;
export type TMixinChange = bindings.MixinChange;
export type TMixinChangeChildrenMixin = bindings.MixinChangeChildrenMixin;
export type TMixinChangeRelativeTransformMixin = bindings.MixinChangeRelativeTransformMixin;
export type TNodeComponent = bindings.Node;
export type TNodeCompositionMixin = bindings.NodeCompositionMixin;
export type TNodeType = bindings.NodeType;
export type TOutputEvent = bindings.OutputEvent;
export type TPaint = bindings.Paint;
export type TPathMixin = bindings.PathMixin;
export type TPolygon = bindings.Polygon;
export type TRectangle = bindings.Rectangle;
export type TRectangleCornerMixin = bindings.RectangleCornerMixin;
export type TRectangleNodeBundle = bindings.RectangleNodeBundle;
export type TRelativeTransformMixin = bindings.RelativeTransformMixin;
export type TRenderChange = bindings.RenderChange;
export type TRenderUpdateEvent = bindings.RenderUpdateEvent;
export type TRoot = bindings.Root;
export type TSVGAttribute = bindings.SVGAttribute;
export type TSVGBlendMode = bindings.SVGBlendMode;
export type TSVGDisplayStyle = bindings.SVGDisplayStyle;
export type TSVGMeasurementUnit = bindings.SVGMeasurementUnit;
export type TSVGPathCommand = bindings.SVGPathCommand;
export type TSVGStyle = bindings.SVGStyle;
export type TSVGTransformAttribute = bindings.SVGTransformAttribute;
export type TSelected = bindings.Selected;
export type TSelectionChangeEvent = bindings.SelectionChangeEvent;
export type TSolidPaint = bindings.SolidPaint;
export type TStar = bindings.Star;
export type TStyleRemoved = bindings.StyleRemoved;
export type TStyleUpdated = bindings.StyleUpdated;
export type TText = bindings.Text;
export type TTextNodeBundle = bindings.TextNodeBundle;
export type TTextSection = bindings.TextSection;
export type TTextStyle = bindings.TextStyle;
export type TTrackUpdateEvent = bindings.TrackUpdateEvent;
export type TTrackableMixinType = bindings.TrackableMixinType;
export type TVec2 = bindings.Vec2;
export type TVerticalTextAlignment = bindings.VerticalTextAlignment;
export type TXYWH = bindings.XYWH;

export { bindings };