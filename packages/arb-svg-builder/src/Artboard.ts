import { shortId } from '@ibg/utils';
import { SvgArbHandle } from '@/rust/dyn-arb-svg-builder-api';
import type {
	ArtboardChangeOutputEvent,
	ComponentChange,
	CoreInputEvent,
	CursorChangeOutputEvent,
	DtifArtboard,
	Entity,
	ExecuteLuaScriptInputEvent,
	InteractionInputEvent,
	InteractionModeChangeOutputEvent,
	InteractionToolChangeOutputEvent,
	LuaScriptError,
	SelectionChangeOutputEvent,
	Size,
	SvgArbInputEvent,
	SvgArbOutputEvent,
	SvgElementChangesOutputEvent,
	Viewport,
	WatchableComponentVariant,
	WatchedEntityChangesOutputEvent
} from '@/rust/dyn-arb-svg-builder-api/bindings';

import { SvgBuilder, type TSvgBuilderOptions } from './SvgBuilder';

export class Artboard {
	private readonly _svgArbHandle: SvgArbHandle;
	private _builder: SvgBuilder;

	private _size: Size;
	private _viewport: Viewport;
	private _selectedEntities: Entity[];

	private _inputEventQueue: SvgArbInputEvent[] = [];
	private _watchedOutputEventCallbackMap: TWatchedOutputEventsCallbackMap = {};

	// Interaction events debounce
	private debounceTimeout: number | null = null;
	private readonly debounceDelay: number = 100; // ms

	constructor(config: TArtboardConfig) {
		const { dtif, interactive = false } = config;
		this._svgArbHandle = SvgArbHandle.create(dtif, interactive);
		this._builder = new SvgBuilder(this, config);
		this.watchOutputEvent('ArtboardChange', (event) => {
			this._size = event.size;
			this._viewport = event.viewport;
		});
		this.watchOutputEvent('SelectionChange', (event) => {
			this._selectedEntities = event.selectedEntities;
		});
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get size(): Readonly<Size> {
		return this._size;
	}

	public get viewport(): Readonly<Viewport> {
		return this._viewport;
	}

	public get selectedEntities(): Readonly<Entity[]> {
		return this._selectedEntities;
	}

	public get builder(): SvgBuilder {
		return this._builder;
	}

	// =========================================================================
	// Lifecycle
	// =========================================================================

	public update(): void {
		const outputEvents: SvgArbOutputEvent[] = this._svgArbHandle.update(
			this._inputEventQueue.splice(0, this._inputEventQueue.length)
		);

		// Handle output events
		for (const event of outputEvents) {
			this.handleOutputEvent(event.type, event);
		}
	}

	public unmount(): void {
		this._svgArbHandle.free();
		this._builder.clear();
		this._watchedOutputEventCallbackMap = {};
		this._inputEventQueue = [];
	}

	// =========================================================================
	// Events
	// =========================================================================

	private handleOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		event: TOutputEventTypeMap[GEventType]
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		callbacks.forEach((entry) => {
			entry.callback(event);
		});
	}

	public watchOutputEvent<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		callback: TWatchedOutputEventCallback<GEventType>
	): () => void {
		const id = shortId();
		const entry: TWatchedOutputEventCallbackEntry<GEventType> = { id, callback };

		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (Array.isArray(callbacks)) {
			callbacks.push(entry);
		} else {
			// @ts-expect-error -- Entry is of type GEventType
			this._watchedOutputEventCallbackMap[eventType] = [entry];
		}

		// Return an unregister function
		return () => {
			this.unregisterOutputEventCallback(eventType, id);
		};
	}

	private unregisterOutputEventCallback<GEventType extends keyof TOutputEventTypeMap>(
		eventType: GEventType,
		id: string
	): void {
		const callbacks = this._watchedOutputEventCallbackMap[eventType];
		if (!Array.isArray(callbacks)) {
			return;
		}

		// @ts-expect-error -- Array has only elements of type GEventType
		this._watchedOutputEventCallbackMap[eventType] = callbacks.filter((entry) => entry.id !== id);
	}

	public watchEntity(
		entity: Entity,
		toWatchComponents: WatchableComponentVariant[],
		callback: TWatchEntityCallback,
		initialValue = true
	): () => void {
		const intialChanges = this._svgArbHandle.watchEntity(entity, toWatchComponents, initialValue);

		// Apply intal changes if found
		if (initialValue && Array.isArray(intialChanges)) {
			callback(entity, intialChanges);
		}

		// Register callback
		const unregister = this.watchOutputEvent('WatchedEntityChange', (event) => {
			if (event.entity === entity) {
				callback(entity, event.changes);
			}
		});

		return () => {
			unregister();
			this._svgArbHandle.unregisterEntityCallback(entity);
		};
	}

	public emitInputEvent<GEventType extends keyof TInputEventTypeMap>(
		eventType: GEventType,
		event: TInputEventTypeMap[GEventType],
		debounce = true
	): void {
		this._inputEventQueue.push({ type: eventType, event } as SvgArbInputEvent);

		// Delay update call, resetting timer on new events within debounceDelay
		if (eventType === 'Interaction' && this.builder.isCallbackBased) {
			if (this.debounceTimeout != null) {
				clearTimeout(this.debounceTimeout);
			}
			if (debounce) {
				this.debounceTimeout = setTimeout(() => {
					this.update();
				}, this.debounceDelay) as unknown as number;
			} else {
				this.update();
			}
		}
	}

	public emitInputEvents<GEventType extends keyof TInputEventTypeMap>(
		eventType: GEventType,
		events: TInputEventTypeMap[GEventType][],
		debounce = true
	): void {
		for (const event of events) {
			this.emitInputEvent(eventType, event, debounce);
		}
	}

	// =========================================================================
	// Other
	// =========================================================================

	public executeScript(script: ExecuteLuaScriptInputEvent): LuaScriptError | null {
		return this._svgArbHandle.executeScript(script);
	}

	public logEntityComponentsRaw(rawEntity: number): void {
		this._svgArbHandle.logEntityComponentsRaw(rawEntity);
	}

	public logEntityComponents(entity: Entity): void {
		this._svgArbHandle.logEntityComponents(entity);
	}

	public tempDevLog(): void {
		this._svgArbHandle.tempDevLog();
	}

	public toString(): string | null {
		return this._svgArbHandle.toString() ?? null;
	}
}

export interface TArtboardConfig extends TSvgBuilderOptions {
	dtif: DtifArtboard;
	interactive?: boolean;
}

export interface TInputEventTypeMap {
	Core: CoreInputEvent;
	Interaction: InteractionInputEvent;
}

export interface TOutputEventTypeMap {
	SvgElementChange: SvgElementChangesOutputEvent;
	WatchedEntityChange: WatchedEntityChangesOutputEvent;
	SelectionChange: SelectionChangeOutputEvent;
	ArtboardChange: ArtboardChangeOutputEvent;
	InteractionModeChange: InteractionModeChangeOutputEvent;
	InteractionToolChange: InteractionToolChangeOutputEvent;
	CursorChange: CursorChangeOutputEvent;
}

export type TWatchedOutputEventCallback<GEventType extends keyof TOutputEventTypeMap> = (
	event: TOutputEventTypeMap[GEventType]
) => void;

interface TWatchedOutputEventCallbackEntry<GEventType extends keyof TOutputEventTypeMap> {
	id: string;
	callback: TWatchedOutputEventCallback<GEventType>;
}

type TWatchedOutputEventsCallbackMap = {
	[K in SvgArbOutputEvent['type']]?: TWatchedOutputEventCallbackEntry<K>[];
};

export type TWatchEntityCallback = (entity: Entity, change: ComponentChange[]) => void;
