import type { TFontMetadata, TFontWithContent } from '@dyn/dtif';

import type { TToTransformFont } from '../../FigmaNodeTreeProcessor';
import type { TContentType, TExportOptions } from '../../types';

export async function transformFont(
	toTransformFont: TToTransformFont,
	config: TTransformFontConfig
): Promise<TFontWithContent> {
	// TODO
	return null as any;
}

export interface TTransformFontConfig {
	exportOptions: TExportOptions;
	resolveFontContent: TResolveFontContent;
}

export type TResolveFontContent = (
	fontMetadata: TFontMetadata
) => Promise<{ content: Uint8Array | null; contentType: TContentType }>;
