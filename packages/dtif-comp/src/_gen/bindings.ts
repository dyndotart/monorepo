 // This file has been generated by Specta. DO NOT EDIT.

/**
 * An absolute length.
 */
export type Abs = Scalar

/**
 * Different units of absolute measurement.
 */
export type AbsUnit = 
/**
 * Points.
 */
"Pt" | 
/**
 * Millimeters.
 */
"Mm" | 
/**
 * Centimeters.
 */
"Cm" | 
/**
 * Inches.
 */
"In"

/**
 * An angle describing a rotation.
 */
export type Angle = Scalar

/**
 * Different units of angular measurement.
 */
export type AngleUnit = 
/**
 * Radians.
 */
"Rad" | 
/**
 * Degrees.
 */
"Deg"

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

export type Color = [number, number, number]

export type CompCoreInputEvent = ({ type: "CompositionResized" } & CompositionResizedInputEvent) | ({ type: "CompositionViewportChanged" } & CompositionViewportChangedInputEvent) | ({ type: "EntityDeleted" } & EntityDeletedInputEvent) | ({ type: "EntityMoved" } & EntityMovedInputEvent) | ({ type: "EntitySetPosition" } & EntitySetPositionInputEvent) | ({ type: "EntitySetRotation" } & EntitySetRotationInputEvent)

export type ComponentChange = { type: "Size"; size: Size } | { type: "Transform"; rotationDeg: number; translation: Vec2 } | { type: "GlobalTransform"; rotationDeg: number; translation: Vec2 }

export type CompositionChangeOutputEvent = { rootNodes: Entity[]; viewport: Viewport; size: Size }

export type CompositionResizedInputEvent = { size: Size }

export type CompositionViewportChangedInputEvent = { viewport: Viewport }

export type Constraint = "Start" | "Center" | "End" | "Stretch" | "Scale"

export type Constraints = { horizontal: Constraint; vertical: Constraint }

export type CornerRadii = [Angle, Angle, Angle, Angle]

export type Cursor = { type: "Default" } | { type: "Grabbing" } | { type: "Crosshair" } | { type: "Resize"; rotationDeg: number } | { type: "Rotate"; rotationDeg: number }

export type CursorChangeOutputEvent = { cursor: Cursor }

export type CursorDownOnCompInputEvent = { position: Vec2; button: MouseButton }

export type CursorDownOnEntityInputEvent = { entity: Entity; position: Vec2; button: MouseButton }

export type CursorDownOnResizeHandleInputEvent = { initialBounds: XYWH; corner: number; rotationRad: number }

export type CursorDownOnRotateHandleInputEvent = { corner: number; initialRotationRad: number }

export type CursorEnteredCompInputEvent = null

export type CursorExitedCompInputEvent = null

export type CursorMovedOnCompInputEvent = { position: Vec2 }

export type CursorUpOnCompInputEvent = { position: Vec2; button: MouseButton }

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

export type EllipseNode = { startingAngle?: number; endingAngle?: number; innerRadiusRatio?: number; translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

/**
 * A length that is relative to the font size.
 * 
 * `1em` is the same as the font size.
 */
export type Em = Scalar

export type Entity = number

export type EntityDeletedInputEvent = { entity: Entity }

export type EntityMovedInputEvent = { entity: Entity; dx: number; dy: number }

export type EntitySetPositionInputEvent = { entity: Entity; x: number; y: number }

export type EntitySetRotationInputEvent = { entity: Entity; rotationDeg: Angle }

export type FillStyle = { paintId: string; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity }

/**
 * A typographic font family.
 */
export type FontFamily = 
/**
 * A serif font.
 */
"Serif" | 
/**
 * A sans-serif font.
 */
"SansSerif" | 
/**
 * A cursive font.
 */
"Cursive" | 
/**
 * A fantasy font.
 */
"Fantasy" | 
/**
 * A monospace font.
 */
"Monospace" | 
/**
 * A custom named font.
 */
{ Named: string }

/**
 * Properties of a single font.
 */
export type FontInfo = { 
/**
 * The typographic font family this font is part of.
 */
family: FontFamily; 
/**
 * Properties that distinguish this font from other fonts in the same
 * family.
 */
variant: FontVariant }

/**
 * The width of a font.
 */
export type FontStretch = number

/**
 * The style of a font.
 */
export type FontStyle = 
/**
 * The default, typically upright style.
 */
"Normal" | 
/**
 * A cursive style with custom letterform.
 */
"Italic" | 
/**
 * Just a slanted version of the normal style.
 */
"Oblique"

/**
 * A font unit.
 * https://fonts.google.com/knowledge/glossary/unit
 */
export type FontUnit = { type: "Abs"; value: Abs } | { type: "Em"; value: Em }

/**
 * Properties that distinguish a font from other fonts in the same family.
 */
export type FontVariant = { 
/**
 * The style of the font (normal / italic / oblique).
 */
style: FontStyle; 
/**
 * How heavy the font is (100 - 900).
 */
weight: FontWeight; 
/**
 * How condensed or expanded the font is (0.5 - 2.0).
 */
stretch: FontStretch }

/**
 * The weight of a font.
 */
export type FontWeight = number

export type FrameNode = { clipContent?: boolean; translation?: Vec2; rotationDeg?: Angle; size: Size; cornerRadii?: CornerRadii; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[]; children?: string[] }

export type GradientColorStop = { 
/**
 * The position of the color stop in the gradient, ranging from 0.0 to 1.0.
 */
position: Ratio; 
/**
 * The color of the stop.
 */
color: Color; 
/**
 * The opacity of the stop.
 */
opacity?: Opacity }

export type GradientPaint = { variant: GradientVariant; stops: GradientColorStop[] }

export type GradientVariant = { type: "Linear"; transform?: Mat3 } | { type: "Radial"; transform?: Mat3 }

export type GroupNode = { translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; children?: string[] }

export type HandleSide = "Top" | "Bottom" | "Left" | "Right"

export type HorizontalTextAlignment = "Start" | "End" | "Left" | "Right" | "Center"

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

export type InteractionInputEvent = ({ type: "KeyDownOnComposition" } & KeyDownOnCompInputEvent) | ({ type: "KeyUpOnComposition" } & KeyUpOnCompInputEvent) | ({ type: "CursorEnteredComposition" }) | ({ type: "CursorExitedComposition" }) | ({ type: "CursorMovedOnComposition" } & CursorMovedOnCompInputEvent) | ({ type: "CursorDownOnComposition" } & CursorDownOnCompInputEvent) | ({ type: "CursorUpOnComposition" } & CursorUpOnCompInputEvent) | ({ type: "MouseWheeledOnComposition" } & MouseWheeledOnCompInputEvent) | ({ type: "CursorDownOnEntity" } & CursorDownOnEntityInputEvent) | ({ type: "CursorDownOnResizeHandle" } & CursorDownOnResizeHandleInputEvent) | ({ type: "CursorDownOnRotateHandle" } & CursorDownOnRotateHandleInputEvent) | ({ type: "InteractionToolChanged" } & InteractionToolChangedInputEvent)

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
{ type: "Resizing"; corner: number; initial_bounds: XYWH; rotation_deg: number } | 
/**
 * When the user is rotating the selected nodes.
 */
{ type: "Rotating"; corner: number; initial_rotation_rad: number; rotation_deg: number } | 
/**
 * When the user plans to insert a new node.
 */
{ type: "Inserting"; origin: Vec2; shape_variant: ShapeVariant; entity: Entity | null }

export type InteractionModeChangeOutputEvent = { interactionMode: InteractionModeLabel }

export type InteractionModeLabel = "None" | "Pressing" | "Translating" | "Resizing" | "Rotating" | "Dragging" | "Inserting"

export type InteractionTool = 
/**
 * When the user wants to select nodes and move them around.
 */
{ type: "Select" } | 
/**
 * When the user wants to insert new shape nodes.
 */
{ type: "Shape"; variant: ShapeVariant }

export type InteractionToolChangeOutputEvent = { interactionTool: InteractionTool }

export type InteractionToolChangedInputEvent = { tool: InteractionTool }

/**
 * The key code of a [`KeyboardInput`].
 * 
 * ## Usage
 * 
 * It is used as the generic `T` value of an [`ButtonInput`] to create a `Res<ButtonInput<KeyCode>>`.
 * 
 * Code representing the location of a physical key
 * This mostly conforms to the UI Events Specification's [`KeyboardEvent.code`] with a few
 * exceptions:
 * - The keys that the specification calls `MetaLeft` and `MetaRight` are named `SuperLeft` and
 * `SuperRight` here.
 * - The key that the specification calls "Super" is reported as `Unidentified` here.
 * 
 * [`KeyboardEvent.code`]: https://w3c.github.io/uievents-code/#code-value-tables
 * 
 * ## Updating
 * 
 * The resource is updated inside of the [`keyboard_input_system`].
 */
export type KeyCode = 
/**
 * This variant is used when the key cannot be translated to any other variant.
 * 
 * The native keycode is provided (if available) so you're able to more reliably match
 * key-press and key-release events by hashing the [`KeyCode`]. It is also possible to use
 * this for keybinds for non-standard keys, but such keybinds are tied to a given platform.
 */
{ Unidentified: NativeKeyCode } | 
/**
 * <kbd>`</kbd> on a US keyboard. This is also called a backtick or grave.
 * This is the <kbd>半角</kbd>/<kbd>全角</kbd>/<kbd>漢字</kbd>
 * (hankaku/zenkaku/kanji) key on Japanese keyboards
 */
"Backquote" | 
/**
 * Used for both the US <kbd>\\</kbd> (on the 101-key layout) and also for the key
 * located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
 * 104- and 106-key layouts.
 * Labeled <kbd>#</kbd> on a UK (102) keyboard.
 */
"Backslash" | 
/**
 * <kbd>[</kbd> on a US keyboard.
 */
"BracketLeft" | 
/**
 * <kbd>]</kbd> on a US keyboard.
 */
"BracketRight" | 
/**
 * <kbd>,</kbd> on a US keyboard.
 */
"Comma" | 
/**
 * <kbd>0</kbd> on a US keyboard.
 */
"Digit0" | 
/**
 * <kbd>1</kbd> on a US keyboard.
 */
"Digit1" | 
/**
 * <kbd>2</kbd> on a US keyboard.
 */
"Digit2" | 
/**
 * <kbd>3</kbd> on a US keyboard.
 */
"Digit3" | 
/**
 * <kbd>4</kbd> on a US keyboard.
 */
"Digit4" | 
/**
 * <kbd>5</kbd> on a US keyboard.
 */
"Digit5" | 
/**
 * <kbd>6</kbd> on a US keyboard.
 */
"Digit6" | 
/**
 * <kbd>7</kbd> on a US keyboard.
 */
"Digit7" | 
/**
 * <kbd>8</kbd> on a US keyboard.
 */
"Digit8" | 
/**
 * <kbd>9</kbd> on a US keyboard.
 */
"Digit9" | 
/**
 * <kbd>=</kbd> on a US keyboard.
 */
"Equal" | 
/**
 * Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
 * Labeled <kbd>\\</kbd> on a UK keyboard.
 */
"IntlBackslash" | 
/**
 * Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
 * Labeled <kbd>\\</kbd> (ro) on a Japanese keyboard.
 */
"IntlRo" | 
/**
 * Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
 * Labeled <kbd>¥</kbd> (yen) on a Japanese keyboard. <kbd>\\</kbd> on a
 * Russian keyboard.
 */
"IntlYen" | 
/**
 * <kbd>a</kbd> on a US keyboard.
 * Labeled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
 */
"KeyA" | 
/**
 * <kbd>b</kbd> on a US keyboard.
 */
"KeyB" | 
/**
 * <kbd>c</kbd> on a US keyboard.
 */
"KeyC" | 
/**
 * <kbd>d</kbd> on a US keyboard.
 */
"KeyD" | 
/**
 * <kbd>e</kbd> on a US keyboard.
 */
"KeyE" | 
/**
 * <kbd>f</kbd> on a US keyboard.
 */
"KeyF" | 
/**
 * <kbd>g</kbd> on a US keyboard.
 */
"KeyG" | 
/**
 * <kbd>h</kbd> on a US keyboard.
 */
"KeyH" | 
/**
 * <kbd>i</kbd> on a US keyboard.
 */
"KeyI" | 
/**
 * <kbd>j</kbd> on a US keyboard.
 */
"KeyJ" | 
/**
 * <kbd>k</kbd> on a US keyboard.
 */
"KeyK" | 
/**
 * <kbd>l</kbd> on a US keyboard.
 */
"KeyL" | 
/**
 * <kbd>m</kbd> on a US keyboard.
 */
"KeyM" | 
/**
 * <kbd>n</kbd> on a US keyboard.
 */
"KeyN" | 
/**
 * <kbd>o</kbd> on a US keyboard.
 */
"KeyO" | 
/**
 * <kbd>p</kbd> on a US keyboard.
 */
"KeyP" | 
/**
 * <kbd>q</kbd> on a US keyboard.
 * Labeled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
 */
"KeyQ" | 
/**
 * <kbd>r</kbd> on a US keyboard.
 */
"KeyR" | 
/**
 * <kbd>s</kbd> on a US keyboard.
 */
"KeyS" | 
/**
 * <kbd>t</kbd> on a US keyboard.
 */
"KeyT" | 
/**
 * <kbd>u</kbd> on a US keyboard.
 */
"KeyU" | 
/**
 * <kbd>v</kbd> on a US keyboard.
 */
"KeyV" | 
/**
 * <kbd>w</kbd> on a US keyboard.
 * Labeled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
 */
"KeyW" | 
/**
 * <kbd>x</kbd> on a US keyboard.
 */
"KeyX" | 
/**
 * <kbd>y</kbd> on a US keyboard.
 * Labeled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
 */
"KeyY" | 
/**
 * <kbd>z</kbd> on a US keyboard.
 * Labeled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
 * QWERTZ (e.g., German) keyboard.
 */
"KeyZ" | 
/**
 * <kbd>-</kbd> on a US keyboard.
 */
"Minus" | 
/**
 * <kbd>.</kbd> on a US keyboard.
 */
"Period" | 
/**
 * <kbd>'</kbd> on a US keyboard.
 */
"Quote" | 
/**
 * <kbd>;</kbd> on a US keyboard.
 */
"Semicolon" | 
/**
 * <kbd>/</kbd> on a US keyboard.
 */
"Slash" | 
/**
 * <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
 */
"AltLeft" | 
/**
 * <kbd>Alt</kbd>, <kbd>Option</kbd>, or <kbd>⌥</kbd>.
 * This is labeled <kbd>AltGr</kbd> on many keyboard layouts.
 */
"AltRight" | 
/**
 * <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
 * Labeled <kbd>Delete</kbd> on Apple keyboards.
 */
"Backspace" | 
/**
 * <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
 */
"CapsLock" | 
/**
 * The application context menu key, which is typically found between the right
 * <kbd>Super</kbd> key and the right <kbd>Control</kbd> key.
 */
"ContextMenu" | 
/**
 * <kbd>Control</kbd> or <kbd>⌃</kbd>
 */
"ControlLeft" | 
/**
 * <kbd>Control</kbd> or <kbd>⌃</kbd>
 */
"ControlRight" | 
/**
 * <kbd>Enter</kbd> or <kbd>↵</kbd>. Labeled <kbd>Return</kbd> on Apple keyboards.
 */
"Enter" | 
/**
 * The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
 */
"SuperLeft" | 
/**
 * The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd>, or other OS symbol key.
 */
"SuperRight" | 
/**
 * <kbd>Shift</kbd> or <kbd>⇧</kbd>
 */
"ShiftLeft" | 
/**
 * <kbd>Shift</kbd> or <kbd>⇧</kbd>
 */
"ShiftRight" | 
/**
 * <kbd> </kbd> (space)
 */
"Space" | 
/**
 * <kbd>Tab</kbd> or <kbd>⇥</kbd>
 */
"Tab" | 
/**
 * Japanese: <kbd>変</kbd> (henkan)
 */
"Convert" | 
/**
 * Japanese: <kbd>カタカナ</kbd>/<kbd>ひらがな</kbd>/<kbd>ローマ字</kbd> (katakana/hiragana/romaji)
 */
"KanaMode" | 
/**
 * Korean: HangulMode <kbd>한/영</kbd> (han/yeong)
 * 
 * Japanese (Mac keyboard): <kbd>か</kbd> (kana)
 */
"Lang1" | 
/**
 * Korean: Hanja <kbd>한</kbd> (hanja)
 * 
 * Japanese (Mac keyboard): <kbd>英</kbd> (eisu)
 */
"Lang2" | 
/**
 * Japanese (word-processing keyboard): Katakana
 */
"Lang3" | 
/**
 * Japanese (word-processing keyboard): Hiragana
 */
"Lang4" | 
/**
 * Japanese (word-processing keyboard): Zenkaku/Hankaku
 */
"Lang5" | 
/**
 * Japanese: <kbd>無変換</kbd> (muhenkan)
 */
"NonConvert" | 
/**
 * <kbd>⌦</kbd>. The forward delete key.
 * Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
 * the keyboard is encoded as [`Backspace`].
 * 
 * [`Backspace`]: Self::Backspace
 */
"Delete" | 
/**
 * <kbd>Page Down</kbd>, <kbd>End</kbd>, or <kbd>↘</kbd>
 */
"End" | 
/**
 * <kbd>Help</kbd>. Not present on standard PC keyboards.
 */
"Help" | 
/**
 * <kbd>Home</kbd> or <kbd>↖</kbd>
 */
"Home" | 
/**
 * <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
 */
"Insert" | 
/**
 * <kbd>Page Down</kbd>, <kbd>PgDn</kbd>, or <kbd>⇟</kbd>
 */
"PageDown" | 
/**
 * <kbd>Page Up</kbd>, <kbd>PgUp</kbd>, or <kbd>⇞</kbd>
 */
"PageUp" | 
/**
 * <kbd>↓</kbd>
 */
"ArrowDown" | 
/**
 * <kbd>←</kbd>
 */
"ArrowLeft" | 
/**
 * <kbd>→</kbd>
 */
"ArrowRight" | 
/**
 * <kbd>↑</kbd>
 */
"ArrowUp" | 
/**
 * On the Mac, this is used for the numpad <kbd>Clear</kbd> key.
 */
"NumLock" | 
/**
 * <kbd>0 Ins</kbd> on a keyboard. <kbd>0</kbd> on a phone or remote control
 */
"Numpad0" | 
/**
 * <kbd>1 End</kbd> on a keyboard. <kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or remote control
 */
"Numpad1" | 
/**
 * <kbd>2 ↓</kbd> on a keyboard. <kbd>2 ABC</kbd> on a phone or remote control
 */
"Numpad2" | 
/**
 * <kbd>3 PgDn</kbd> on a keyboard. <kbd>3 DEF</kbd> on a phone or remote control
 */
"Numpad3" | 
/**
 * <kbd>4 ←</kbd> on a keyboard. <kbd>4 GHI</kbd> on a phone or remote control
 */
"Numpad4" | 
/**
 * <kbd>5</kbd> on a keyboard. <kbd>5 JKL</kbd> on a phone or remote control
 */
"Numpad5" | 
/**
 * <kbd>6 →</kbd> on a keyboard. <kbd>6 MNO</kbd> on a phone or remote control
 */
"Numpad6" | 
/**
 * <kbd>7 Home</kbd> on a keyboard. <kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
 * or remote control
 */
"Numpad7" | 
/**
 * <kbd>8 ↑</kbd> on a keyboard. <kbd>8 TUV</kbd> on a phone or remote control
 */
"Numpad8" | 
/**
 * <kbd>9 PgUp</kbd> on a keyboard. <kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
 * or remote control
 */
"Numpad9" | 
/**
 * <kbd>+</kbd>
 */
"NumpadAdd" | 
/**
 * Found on the Microsoft Natural Keyboard.
 */
"NumpadBackspace" | 
/**
 * <kbd>C</kbd> or <kbd>A</kbd> (All Clear). Also for use with numpads that have a
 * <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the
 * numpad <kbd>Clear</kbd> key is encoded as [`NumLock`].
 * 
 * [`NumLock`]: Self::NumLock
 */
"NumpadClear" | 
/**
 * <kbd>C</kbd> (Clear Entry)
 */
"NumpadClearEntry" | 
/**
 * <kbd>,</kbd> (thousands separator). For locales where the thousands separator
 * is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
 */
"NumpadComma" | 
/**
 * <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
 * Brazil), this key may generate a <kbd>,</kbd>.
 */
"NumpadDecimal" | 
/**
 * <kbd>/</kbd>
 */
"NumpadDivide" | 
/**
 * The Enter key on the numpad.
 */
"NumpadEnter" | 
/**
 * <kbd>=</kbd>
 */
"NumpadEqual" | 
/**
 * <kbd>#</kbd> on a phone or remote control device. This key is typically found
 * below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
 */
"NumpadHash" | 
/**
 * <kbd>M</kbd> Add current entry to the value stored in memory.
 */
"NumpadMemoryAdd" | 
/**
 * <kbd>M</kbd> Clear the value stored in memory.
 */
"NumpadMemoryClear" | 
/**
 * <kbd>M</kbd> Replace the current entry with the value stored in memory.
 */
"NumpadMemoryRecall" | 
/**
 * <kbd>M</kbd> Replace the value stored in memory with the current entry.
 */
"NumpadMemoryStore" | 
/**
 * <kbd>M</kbd> Subtract current entry from the value stored in memory.
 */
"NumpadMemorySubtract" | 
/**
 * <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
 * operations (<kbd>+</kbd>, <kbd>-</kbd> <kbd>*</kbd> and <kbd>/</kbd>).
 * 
 * Use `NumpadStar` for the <kbd>*</kbd> key on phones and remote controls.
 */
"NumpadMultiply" | 
/**
 * <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
 */
"NumpadParenLeft" | 
/**
 * <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
 */
"NumpadParenRight" | 
/**
 * <kbd>*</kbd> on a phone or remote control device.
 * 
 * This key is typically found below the <kbd>7</kbd> key and to the left of
 * the <kbd>0</kbd> key.
 * 
 * Use <kbd>"NumpadMultiply"</kbd> for the <kbd>*</kbd> key on
 * numeric keypads.
 */
"NumpadStar" | 
/**
 * <kbd>-</kbd>
 */
"NumpadSubtract" | 
/**
 * <kbd>Esc</kbd> or <kbd>⎋</kbd>
 */
"Escape" | 
/**
 * <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate code.
 */
"Fn" | 
/**
 * <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
 * Natural Keyboard.
 */
"FnLock" | 
/**
 * <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
 */
"PrintScreen" | 
/**
 * <kbd>Scroll Lock</kbd>
 */
"ScrollLock" | 
/**
 * <kbd>Pause Break</kbd>
 */
"Pause" | 
/**
 * Some laptops place this key to the left of the <kbd>↑</kbd> key.
 * 
 * This also the "back" button (triangle) on Android.
 */
"BrowserBack" | 
/**
 * BrowserFavorites
 */
"BrowserFavorites" | 
/**
 * Some laptops place this key to the right of the <kbd>↑</kbd> key.
 */
"BrowserForward" | 
/**
 * The "home" button on Android.
 */
"BrowserHome" | 
/**
 * BrowserRefresh
 */
"BrowserRefresh" | 
/**
 * BrowserSearch
 */
"BrowserSearch" | 
/**
 * BrowserStop
 */
"BrowserStop" | 
/**
 * <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function section on some Apple
 * keyboards.
 */
"Eject" | 
/**
 * Sometimes labelled <kbd>My Computer</kbd> on the keyboard
 */
"LaunchApp1" | 
/**
 * Sometimes labelled <kbd>Calculator</kbd> on the keyboard
 */
"LaunchApp2" | 
/**
 * LaunchMail
 */
"LaunchMail" | 
/**
 * MediaPlayPause
 */
"MediaPlayPause" | 
/**
 * MediaSelect
 */
"MediaSelect" | 
/**
 * MediaStop
 */
"MediaStop" | 
/**
 * MediaTrackNext
 */
"MediaTrackNext" | 
/**
 * MediaTrackPrevious
 */
"MediaTrackPrevious" | 
/**
 * This key is placed in the function section on some Apple keyboards, replacing the
 * <kbd>Eject</kbd> key.
 */
"Power" | 
/**
 * Sleep
 */
"Sleep" | 
/**
 * AudioVolumeDown
 */
"AudioVolumeDown" | 
/**
 * AudioVolumeMute
 */
"AudioVolumeMute" | 
/**
 * AudioVolumeUp
 */
"AudioVolumeUp" | 
/**
 * WakeUp
 */
"WakeUp" | 
/**
 * Legacy modifier key. Also called "Super" in certain places.
 */
"Meta" | 
/**
 * Legacy modifier key.
 */
"Hyper" | 
/**
 * Turbo
 */
"Turbo" | 
/**
 * Abort
 */
"Abort" | 
/**
 * Resume
 */
"Resume" | 
/**
 * Suspend
 */
"Suspend" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Again" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Copy" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Cut" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Find" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Open" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Paste" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Props" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Select" | 
/**
 * Found on Sun’s USB keyboard.
 */
"Undo" | 
/**
 * Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
 */
"Hiragana" | 
/**
 * Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
 */
"Katakana" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F1" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F2" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F3" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F4" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F5" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F6" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F7" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F8" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F9" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F10" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F11" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F12" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F13" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F14" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F15" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F16" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F17" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F18" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F19" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F20" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F21" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F22" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F23" | 
/**
 * General-purpose function key.
 * Usually found at the top of the keyboard.
 */
"F24" | 
/**
 * General-purpose function key.
 */
"F25" | 
/**
 * General-purpose function key.
 */
"F26" | 
/**
 * General-purpose function key.
 */
"F27" | 
/**
 * General-purpose function key.
 */
"F28" | 
/**
 * General-purpose function key.
 */
"F29" | 
/**
 * General-purpose function key.
 */
"F30" | 
/**
 * General-purpose function key.
 */
"F31" | 
/**
 * General-purpose function key.
 */
"F32" | 
/**
 * General-purpose function key.
 */
"F33" | 
/**
 * General-purpose function key.
 */
"F34" | 
/**
 * General-purpose function key.
 */
"F35"

export type KeyDownOnCompInputEvent = { 
/**
 * The physical key code of the key.
 */
keyCode: KeyCode }

export type KeyUpOnCompInputEvent = { 
/**
 * The physical key code of the key.
 */
keyCode: KeyCode }

export type LineWrap = 
/**
 * No wrapping
 */
"None" | 
/**
 * Wraps at a glyph level
 */
"Glyph" | 
/**
 * Wraps at the word level
 */
"Word" | 
/**
 * Wraps at the word level, or fallback to glyph level if a word can't fit on a line by itself
 */
"WordOrGlyph"

export type Mat3 = [number, number, number, number, number, number, number, number, number]

/**
 * A button on a mouse device.
 * 
 * ## Usage
 * 
 * It is used as the generic `T` value of an [`ButtonInput`] to create a `bevy`
 * resource.
 * 
 * ## Updating
 * 
 * The resource is updated inside of the [`mouse_button_input_system`].
 */
export type MouseButton = 
/**
 * The left mouse button.
 */
"Left" | 
/**
 * The right mouse button.
 */
"Right" | 
/**
 * The middle mouse button.
 */
"Middle" | 
/**
 * The back mouse button.
 */
"Back" | 
/**
 * The forward mouse button.
 */
"Forward" | 
/**
 * Another mouse button with the associated number.
 */
{ Other: number }

export type MouseButtonOnEntity = { button: MouseButton; entity: Entity }

export type MouseButtonOnResizeHandle = { button: MouseButton; corner: number }

export type MouseButtonOnResizeHandleValue = { initial_bounds: XYWH; rotation_rad: number }

export type MouseButtonOnRotateHandle = { button: MouseButton; corner: number }

export type MouseButtonOnRotateHandleValue = { initial_rotation_rad: number }

export type MouseButtonValue = { position: Vec2 }

export type MouseWheeledOnCompInputEvent = { position: Vec2; delta: Vec2 }

/**
 * Contains the platform-native physical key identifier
 * 
 * The exact values vary from platform to platform (which is part of why this is a per-platform
 * enum), but the values are primarily tied to the key's physical location on the keyboard.
 * 
 * This enum is primarily used to store raw keycodes when Winit doesn't map a given native
 * physical key identifier to a meaningful [`KeyCode`] variant. In the presence of identifiers we
 * haven't mapped for you yet, this lets you use use [`KeyCode`] to:
 * 
 * - Correctly match key press and release events.
 * - On non-web platforms, support assigning keybinds to virtually any key through a UI.
 */
export type NativeKeyCode = 
/**
 * Unidentified
 */
"unidentified" | 
/**
 * An Android "scancode".
 */
{ android: number } | 
/**
 * A macOS "scancode".
 */
{ macOS: number } | 
/**
 * A Windows "scancode".
 */
{ windows: number } | 
/**
 * An XKB "keycode".
 */
{ xkb: number }

export type Node = ({ type: "Frame" } & FrameNode) | ({ type: "Group" } & GroupNode) | ({ type: "Rectangle" } & RectangleNode) | ({ type: "Ellipse" } & EllipseNode) | ({ type: "Star" } & StarNode) | ({ type: "Polygon" } & PolygonNode) | ({ type: "Text" } & TextNode) | ({ type: "Vector" } & VectorNode)

/**
 * An opacity.
 */
export type Opacity = Ratio

export type Paint = ({ type: "Solid" } & SolidPaint) | ({ type: "Image" } & ImagePaint) | ({ type: "Gradient" } & GradientPaint)

export type PolygonNode = { pointCount?: number; translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

/**
 * A ratio of a whole.
 * 
 * Written as a number, followed by a percent sign.
 */
export type Ratio = Scalar

export type RectangleNode = { translation?: Vec2; rotationDeg?: Angle; size: Size; cornerRadii?: CornerRadii; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

/**
 * A 32-bit float that implements `Eq`, `Ord` and `Hash`.
 * 
 * Panics if it's `NaN` during any of those operations.
 */
export type Scalar = number

export type SelectionChangeOutputEvent = { selectedEntities: Entity[] }

export type ShapeVariant = "Rectangle" | "Ellipse" | "Star" | "Polygon"

/**
 * An absolute size in 2D with a width and a height.
 */
export type Size = [Abs, Abs]

export type SolidPaint = { color: Color }

export type SpectaExport = { comp_dtif: DtifComposition; svg_comp_input_event: SvgCompInputEvent; svg_comp_output_event: SvgCompOutputEvent }

export type StarNode = { innerRadiusRatio?: number; pointCount?: number; translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

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

export type SvgCompInputEvent = { type: "Composition"; event: CompCoreInputEvent } | { type: "Interaction"; event: InteractionInputEvent }

export type SvgCompOutputEvent = ({ type: "SvgElementChange" } & SvgElementChangesOutputEvent) | ({ type: "CompositionChange" } & CompositionChangeOutputEvent) | ({ type: "WatchedEntityChange" } & WatchedEntityChangesOutputEvent) | ({ type: "SelectionChange" } & SelectionChangeOutputEvent) | ({ type: "InteractionModeChange" } & InteractionModeChangeOutputEvent) | ({ type: "InteractionToolChange" } & InteractionToolChangeOutputEvent) | ({ type: "CursorChange" } & CursorChangeOutputEvent)

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

export type TextAttributeInterval = { start: number; end: number; attributes: TextAttributes }

export type TextAttributes = { fontFamily?: FontFamily | null; fontStyle?: FontStyle | null; fontStretch?: FontStretch | null; fontWeight?: FontWeight | null; fontSize?: Abs | null; smallCaps?: boolean | null; applyKerning?: boolean | null; letterSpacing?: FontUnit | null; wordSpacing?: FontUnit | null; lineHeight?: FontUnit | null }

export type TextNode = { text: string; attributes: TextAttributeInterval[]; lineWrap?: LineWrap; horizontalTextAlignment?: HorizontalTextAlignment; verticalTextAlignment?: VerticalTextAlignment; translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

export type Vec2 = [number, number]

export type VectorNode = { path: string; translation?: Vec2; rotationDeg?: Angle; size: Size; visible?: boolean; blendMode?: BlendMode; opacity?: Opacity; constraints?: Constraints; styles?: Style[] }

export type VerticalTextAlignment = "Top" | "Bottom" | "Center"

export type Viewport = { physicalPosition: Vec2; physicalSize: Size }

export type WatchableComponentVariant = "Size" | "Transform" | "GlobalTransform"

export type WatchedEntityChangesOutputEvent = { entity: Entity; changes: ComponentChange[] }

export type XYWH = { position: Vec2; size: Size }

