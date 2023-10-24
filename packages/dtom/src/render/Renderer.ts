import type { Composition } from '../composition';

export abstract class Renderer {
	private _composition: () => Composition; // TODO: Bad practice?

	constructor(composition: Composition) {
		this._composition = () => composition;
	}

	// =========================================================================
	// Getter & Setter
	// =========================================================================

	protected get composition(): Composition {
		return this._composition();
	}

	// =========================================================================
	// Abstract
	// =========================================================================

	public abstract setSize(width: number, height: number): this;

	public abstract render(data: unknown): this;
}
