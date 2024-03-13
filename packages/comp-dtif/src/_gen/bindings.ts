 // This file has been generated by Specta. DO NOT EDIT.

export type Asset = { content: AssetContent; contentType: AssetContentType }

export type AssetContent = 
/**
 * Content stored as binary data.
 */
{ type: "Binary"; content: number[] } | 
/**
 * Content referenced by a URL.
 */
{ type: "Url"; url: string }

export type AssetContentType = { type: "Unknown" } | { type: "Jpeg" } | { type: "Png" } | { type: "Svg"; width: number; height: number } | { type: "Ttf" }

export type BlendMode = "Normal" | "Multiply" | "Screen" | "Overlay" | "Darken" | "Lighten" | "ColorDodge" | "ColorBurn" | "HardLight" | "SoftLight" | "Difference" | "Exclusion" | "Hue" | "Saturation" | "Color" | "Luminosity"

export type BreakLineOn = "WordBoundary" | "AnyCharacter" | "NoWrap"

export type Centimeter = number

export type Color = { red: number; green: number; blue: number }

export type CompCoreInputEvent = ({ type: "CompositionResized" } & CompositionResizedInputEvent) | ({ type: "CompositionViewportChanged" } & CompositionViewportChangedInputEvent) | ({ type: "EntityMoved" } & EntityMovedInputEvent) | ({ type: "EntitySetPosition" } & EntitySetPositionInputEvent) | ({ type: "EntityDeleted" } & EntityDeletedInputEvent)

export type ComponentChange = { type: "Size"; size: Size } | { type: "Transform" }

export type CompositionChangeOutputEvent = { rootNodes: Entity[]; viewport: Viewport; size: Size }

export type CompositionResizedInputEvent = { size: Size }

export type CompositionViewportChangedInputEvent = { viewport: Viewport }

export type CornerRadii = Vec4

export type CursorDownOnCompInputEvent = { position: Vec2; button: MouseButton }

export type CursorDownOnEntityInputEvent = { entity: Entity; position: Vec2; button: MouseButton }

export type CursorDownOnResizeHandleInputEvent = { initialBounds: XYWH; corner: number; rotationInRadians: number }

export type CursorDownOnRotateHandleInputEvent = { corner: number; initialRotationInRadians: number }

export type CursorEnteredCompInputEvent = null

export type CursorExitedCompInputEvent = null

export type CursorMovedOnCompInputEvent = { position: Vec2 }

export type CursorUpOnCompInputEvent = { position: Vec2; button: MouseButton }

export type Degree = number

/**
 * DTIF (Design Tree Interchange Format) utilizes a flat structure for easy readability
 * and efficient access & manipulation of design elements (Nodes, Paints, ..).
 * https://softwareengineering.stackexchange.com/questions/350623/flat-or-nested-json-for-hierarchal-data
 */
export type DtifComposition = { 
/**
 * The version of the composition type declaration.
 */
version?: string | null; 
/**
 * The size of the composition in pixels.
 */
size: Size; 
/**
 * The viewport defines the area on the render target to which the camera renders its image.
 */
viewport?: Viewport | null; 
/**
 * The identifier of the root node in the composition.
 */
rootNodeId: string; 
/**
 * A mapping of node identifiers to their corresponding nodes within the composition.
 */
nodes: { [key in string]: Node }; 
/**
 * A mapping of paint identifiers to their corresponding paints within the composition.
 */
paints?: { [key in string]: Paint }; 
/**
 * A mapping of asset identifiers to their corresponding assets within the composition.
 */
assets?: { [key in string]: Asset }; events?: DtifInputEvent[] }

export type DtifCompositionResizedEvent = { size: Size }

export type DtifCompositionViewportChangedEvent = { viewport: Viewport }

export type DtifEntityDeletedEvent = { entity: string }

export type DtifEntityMovedEvent = { entity: string; dx: number; dy: number }

export type DtifEntitySetPositionEvent = { entity: string; x: number; y: number }

export type DtifInputEvent = ({ type: "CompositionResized" } & DtifCompositionResizedEvent) | ({ type: "CompositionViewportChanged" } & DtifCompositionViewportChangedEvent) | ({ type: "EntityMoved" } & DtifEntityMovedEvent) | ({ type: "EntitySetPosition" } & DtifEntitySetPositionEvent) | ({ type: "EntityDeleted" } & DtifEntityDeletedEvent)

export type EllipseNode = { startingAngle?: number; endingAngle?: number; innerRadiusRatio?: number; translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

export type Entity = number

export type EntityDeletedInputEvent = { entity: Entity }

export type EntityMovedInputEvent = { entity: Entity; dx: number; dy: number }

export type EntitySetPositionInputEvent = { entity: Entity; x: number; y: number }

export type FillStyle = { paintId: string; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity }

export type FontMetadata = { 
/**
 * The font family to which this font belongs.
 */
family: string; 
/**
 * The style of the font, such as italic or normal.
 */
style: FontStyle; 
/**
 * The weight of the font, typically ranging from 100 (thin) to 900 (black).
 */
weight: number }

export type FontStyle = 
/**
 * A face that is neither italic not obliqued.
 */
"Normal" | 
/**
 * A form that is generally cursive in nature.
 */
"Italic" | 
/**
 * A typically-sloped version of the regular face.
 */
"Oblique"

export type FrameNode = { clipContent?: boolean; translation?: Vec2; angle?: Degree; size: Size; cornerRadii?: CornerRadii; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[]; children?: string[] }

export type GradientColorStop = { 
/**
 * The position of the color stop in the gradient, ranging from 0.0 to 1.0.
 */
position: Percent; 
/**
 * The color of the stop.
 */
color: Color; 
/**
 * The opacity of the stop.
 */
opacity?: Percent }

export type GradientPaint = { variant: GradientVariant; stops: GradientColorStop[] }

export type GradientVariant = { type: "Linear"; transform?: Mat3 } | { type: "Radial"; transform?: Mat3 }

export type GroupNode = { translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; children?: string[] }

export type HandleSide = "Top" | "Bottom" | "Left" | "Right"

export type HorizontalTextAlignment = "Left" | "Center" | "Right" | "Justified"

export type ImagePaint = { assetId: string; scaleMode?: ImageScaleMode }

export type ImageScaleMode = 
/**
 * Fills the area completely with the image.
 */
{ type: "Fill" } | 
/**
 * Fits the image within the area while maintaining its aspect ratio.
 */
{ type: "Fit" } | 
/**
 * Crops the image to fill the area.
 */
{ type: "Crop"; transform?: Mat3 } | 
/**
 * Tiles the image within the area.
 */
{ type: "Tile"; rotation?: number; scalingFactor: number }

export type Inch = number

export type InteractionInputEvent = ({ type: "CursorDownOnEntity" } & CursorDownOnEntityInputEvent) | ({ type: "CursorMovedOnComposition" } & CursorMovedOnCompInputEvent) | ({ type: "CursorEnteredComposition" }) | ({ type: "CursorExitedComposition" }) | ({ type: "CursorDownOnComposition" } & CursorDownOnCompInputEvent) | ({ type: "CursorUpOnComposition" } & CursorUpOnCompInputEvent) | ({ type: "WheeledOnComposition" } & WheeledOnCompInputEvent) | ({ type: "CursorDownOnResizeHandle" } & CursorDownOnResizeHandleInputEvent) | ({ type: "CursorDownOnRotateHandle" } & CursorDownOnRotateHandleInputEvent)

export type InteractionMode = 
/**
 * Default canvas mode. Nothing is happening.
 */
{ type: "None" } | 
/**
 * When the user's pointer is pressed.
 */
{ type: "Pressing"; origin: Vec2; button: MouseButton } | 
/**
 * When the user is dragging.
 */
{ type: "Dragging"; current: Vec2 } | 
/**
 * When the user is moving selected nodes.
 */
{ type: "Translating"; origin: Vec2; current: Vec2 } | 
/**
 * When the user is resizing the selected nodes.
 */
{ type: "Resizing"; corner: number; initial_bounds: XYWH; rotation_in_degrees: number } | 
/**
 * When the user is rotating the selected nodes.
 */
{ type: "Rotating"; corner: number; initial_rotation_in_radians: number; rotation_in_degrees: number }

export type LetterSpacing = "Auto" | { Fixed: MeasurementUnit }

export type LineHeight = "Auto" | { Fixed: MeasurementUnit }

export type Mat3 = [number, number, number, number, number, number, number, number, number]

export type MeasurementUnit = { type: "Pixels"; pixels: Pixel } | { type: "Percent"; percent: Percent } | { type: "Inch"; inch: Inch } | { type: "Centimeter"; centimeter: Centimeter } | { type: "Millimeter"; millimeter: Millimeter }

export type Millimeter = number

export type MouseButton = "Left" | "Middle" | "Right" | "Unkown"

export type Node = ({ type: "Frame" } & FrameNode) | ({ type: "Group" } & GroupNode) | ({ type: "Rectangle" } & RectangleNode) | ({ type: "Ellipse" } & EllipseNode) | ({ type: "Star" } & StarNode) | ({ type: "Polygon" } & PolygonNode) | ({ type: "Text" } & TextNode) | ({ type: "Vector" } & VectorNode)

export type Opacity = Percent

export type Paint = ({ type: "Solid" } & SolidPaint) | ({ type: "Image" } & ImagePaint) | ({ type: "Gradient" } & GradientPaint)

export type Percent = number

export type Pixel = number

export type PolygonNode = { pointCount?: number; translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

export type RectangleNode = { translation?: Vec2; angle?: Degree; size: Size; cornerRadii?: CornerRadii; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

export type SelectionChangeOutputEvent = { selected: Entity[] }

export type Size = Vec2

export type SolidPaint = { color: Color }

export type SpectaExport = { comp_dtif: DtifComposition; svg_comp_input_event: SvgCompInputEvent; svg_comp_output_event: SvgCompOutputEvent }

export type StarNode = { innerRadiusRatio?: number; pointCount?: number; translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

export type StrokeStyle = { width: number; paintId: string; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity }

export type Style = ({ type: "Fill" } & FillStyle) | ({ type: "Stroke" } & StrokeStyle)

export type SvgAttribute = { type: "Id"; id: SvgElementId } | { type: "Class"; class: string } | { type: "Width"; width: number; unit: SvgMeasurementUnit } | { type: "Height"; height: number; unit: SvgMeasurementUnit } | { type: "Transform"; transform: SvgTransformAttribute } | { type: "PatternTransform"; patternTransform: SvgTransformAttribute } | { type: "D"; d: SvgPathAttribute } | { type: "ClipPath"; clipPath: SvgElementId } | { type: "Fill"; fill: SvgAttributeColor } | { type: "PatternUnits"; patternUnits: SvgUnits } | { type: "GradientUnits"; gradientUnits: SvgUnits } | { type: "Href"; href: SvgHrefAttribute } | { type: "PreserveAspectRatio"; preserveAspectRatio: string } | { type: "X1"; x1: number } | { type: "Y1"; y1: number } | { type: "X2"; x2: number } | { type: "Y2"; y2: number } | { type: "Offset"; offset: number } | { type: "StopColor"; stopColor: SvgAttributeColor } | { type: "StopOpacity"; stopOpacity: number }

export type SvgAttributeColor = { RGB: { red: number; green: number; blue: number } } | { RGBA: { red: number; green: number; blue: number; alpha: number } } | { Reference: { id: SvgElementId } } | "None"

/**
 * Emitted when an attribute of a SvgElement is removed.
 */
export type SvgAttributeRemovedChange = { key: string }

/**
 * Emitted when an attribute of an SvgElement is updated.
 */
export type SvgAttributeUpdatedChange = { key: string; newValue: string }

export type SvgBlendModeStyle = "Normal" | "Multiply" | "Screen" | "Overlay" | "Darken" | "Lighten" | "ColorDodge" | "ColorBurn" | "HardLight" | "SoftLight" | "Difference" | "Exclusion" | "Hue" | "Saturation" | "Color" | "Luminosity"

export type SvgBuilderOutputEvent = 
/**
 * Represents incremental changes to an SVG element.
 * Emitted when the "output_svg_element_changes" feature is enabled.
 */
({ type: "SvgElementChanges" } & SvgElementChangesOutputEvent) | 
/**
 * Contains the complete SVG as a string.
 * Emitted when the "output_svg_string" feature is enabled.
 */
({ type: "SvgString" } & SvgStringOutputEvent)

export type SvgCompInputEvent = { type: "Comp"; event: CompCoreInputEvent } | { type: "Interaction"; event: InteractionInputEvent }

export type SvgCompOutputEvent = ({ type: "SvgElementChanges" } & SvgElementChangesOutputEvent) | ({ type: "CompositionChange" } & CompositionChangeOutputEvent) | ({ type: "WatchedEntityChanges" } & WatchedEntityChangesOutputEvent) | ({ type: "SelectionChange" } & SelectionChangeOutputEvent)

export type SvgDisplayStyle = "Block" | "None"

/**
 * Emitted when a SvgElement (child) is append to another SvgElement (parent).
 */
export type SvgElementAppendedChange = { parentId: SvgElementId }

export type SvgElementChange = ({ type: "ElementCreated" } & SvgElementCreatedChange) | ({ type: "ElementDeleted" }) | ({ type: "ElementAppended" } & SvgElementAppendedChange) | ({ type: "AttributeUpdated" } & SvgAttributeUpdatedChange) | ({ type: "AttributeRemoved" } & SvgAttributeRemovedChange) | ({ type: "StyleUpdated" } & SvgStyleUpdatedChange) | ({ type: "StyleRemoved" } & SvgStyleRemovedChange) | ({ type: "ElementChildrenReordered" } & SvgElementChildrenReorderedChange)

export type SvgElementChangesOutputEvent = { id: SvgElementId; changes: SvgElementChange[] }

/**
 * Emitted when children of a SvgElement are reordered.
 */
export type SvgElementChildrenReorderedChange = { newOrder: SvgElementId[] }

/**
 * Emitted when a new SvgElement is created.
 */
export type SvgElementCreatedChange = { tagName: string; attributes: ([string, string])[]; styles: ([string, string])[]; parentId: SvgElementId | null; entity: Entity | null }

/**
 * Emitted when a new SvgElement is deleted.
 */
export type SvgElementDeletedChange = Record<string, never>

export type SvgElementId = number

export type SvgHrefAttribute = { Base64: { content: string; contentType: SvgHrefContentType } } | { Url: { url: string } }

export type SvgHrefContentType = "Jpeg" | "Png" | "Svg"

export type SvgMeasurementUnit = "Pixel" | "Percent"

export type SvgPathAttribute = string

export type SvgPointerEventsStyle = "None" | "All"

export type SvgStringOutputEvent = { value: string }

export type SvgStyle = { type: "Display"; display: SvgDisplayStyle } | { type: "BlendMode"; blendMode: SvgBlendModeStyle } | { type: "Opacity"; opacity: number } | { type: "Fill"; fill: SvgStyleColor } | { type: "Stroke"; stroke: SvgStyleColor } | { type: "StrokeWidth"; strokeWidth: number } | { type: "StrokeOpacity"; strokeOpacity: number } | { type: "PointerEvents"; pointerEvents: SvgPointerEventsStyle }

export type SvgStyleColor = { RGB: { red: number; green: number; blue: number } } | { RGBA: { red: number; green: number; blue: number; alpha: number } } | { Reference: { id: SvgElementId } } | "None"

/**
 * Emitted when a style property of a SvgElement is removed.
 */
export type SvgStyleRemovedChange = { key: string }

/**
 * Emitted when a style property of a SvgElement is updated.
 */
export type SvgStyleUpdatedChange = { key: string; newValue: string }

export type SvgTransformAttribute = { type: "Matrix"; a: number; b: number; c: number; d: number; tx: number; ty: number } | { type: "Rotate"; rotation: number }

export type SvgUnits = "UserSpaceOnUse" | "ObjectBoundingBox"

export type TextNode = { spans: TextSpan[]; horizontalTextAlignment?: HorizontalTextAlignment; verticalTextAlignment?: VerticalTextAlignment; linebreakBehavior?: BreakLineOn; translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

/**
 * A styled text segment.
 */
export type TextSpan = { 
/**
 * Text content.
 */
text: string; 
/**
 * Font metadata.
 */
font: FontMetadata; 
/**
 * Style properties.
 */
style: TextStyle }

/**
 * Style properties for a text segment.
 */
export type TextStyle = { 
/**
 * Glyph height in pixels, may scale with window.
 */
fontSize: number; 
/**
 * Character spacing.
 */
letterSpacing: LetterSpacing; 
/**
 * Line spacing.
 */
lineHeight: LineHeight }

export type Vec2 = [number, number]

export type Vec4 = [number, number, number, number]

export type VectorNode = { path: string; translation?: Vec2; angle?: Degree; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; styles?: Style[] }

export type VerticalTextAlignment = "Top" | "Center" | "Bottom" | "Justified"

export type Viewport = { physicalPosition: Vec2; physicalSize: Vec2 }

export type WatchableComponentVariant = "Size" | "Transform"

export type WatchedEntityChangesOutputEvent = { entity: Entity; changes: ComponentChange[] }

export type WheeledOnCompInputEvent = { position: Vec2; delta: Vec2; ctrlKeyPressed: boolean; metaKeyPressed: boolean }

export type XYWH = { position: Vec2; width: number; height: number }

