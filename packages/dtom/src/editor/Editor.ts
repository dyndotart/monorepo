import { Editor as RustEditor } from '@rust/dyn-dtom';

import type { Renderer } from '../render';
import type { RenderUpdate, ToJsEvent, WorldIds } from '../wasm';

export class Editor {
	private _rustEditor: RustEditor;
	private _worldIds: TWorldIds;

	// Keep track of all instances of Editor
	// so we can pass WASM events to the correct instance based on the worldId
	private static _INSTANCES: Editor[] = [];

	private _renderer: Renderer[] = [];

	protected _width: number;
	protected _height: number;

	constructor(config: TEditorConfig) {
		const { width, height } = config;
		this._rustEditor = new RustEditor();
		this._worldIds = this.retrieveWorldIds();
		this._width = width;
		this._height = height;

		Editor._INSTANCES.push(this);
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	public get worldIds(): TWorldIds {
		return this._worldIds;
	}

	// =========================================================================
	// WASM interface
	// =========================================================================

	private retrieveWorldIds(): TWorldIds {
		const worldIds: WorldIds = this._rustEditor.get_world_ids();
		return {
			mainWorldId: worldIds.main_world_id,
			renderWorldId: worldIds.render_world_id
		};
	}

	public static onWasmEvent(worldId: number, data: ToJsEvent): void {
		Editor._INSTANCES.find((instance) => instance.hasWorldId(worldId))?.onWasmEvent(data);
	}

	public onWasmEvent(event: ToJsEvent): this {
		for (const [key, value] of Object.entries(event)) {
			switch (key) {
				case 'RenderUpdate':
					this.onRenderUpdate(value as RenderUpdate);
					break;
				default:
					console.warn(`Unknown event: ${key}`);
					break;
			}
		}
		return this;
	}

	private onRenderUpdate(data: RenderUpdate): this {
		this._renderer.forEach((renderer) => renderer.render(data));
		return this;
	}

	// =========================================================================
	// Interface
	// =========================================================================

	public setSize(width: number, height: number): this {
		this._width = width;
		this._height = height;
		this._renderer.forEach((renderer) => renderer.setSize(width, height));
		return this;
	}

	public registerRenderer(renderer: Renderer): this {
		renderer.setSize(this._width, this._height);
		this._renderer.push(renderer);
		return this;
	}

	public createRect(): void {
		this._rustEditor.create_rect();
	}

	public hasWorldId(worldId: number): boolean {
		return Object.values(this._worldIds).includes(worldId);
	}

	public update(): void {
		this._rustEditor.update();
	}
}

export interface TEditorConfig {
	width: number;
	height: number;
}

export interface TWorldIds {
	mainWorldId: number;
	renderWorldId: number;
}
