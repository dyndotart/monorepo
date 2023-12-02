import { Font, FontWithContent } from '@dyn/svg-composition';

export const INTER_REGULAR: TFont = {
	hash: 123,
	meta: {
		name: 'Inter Regular',
		family: 'Inter',
		style: 'Normal',
		weight: 400
	},
	url: 'https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfMZhrib2Bg-4.ttf'
};

export async function loadFont(font: TFont): Promise<FontWithContent> {
	const response = await fetch(font.url);
	const arrayBuffer = await response.arrayBuffer();
	return {
		font: font.meta,
		content: Array.from(new Uint8Array(arrayBuffer)),
		hash: font.hash
	};
}

type TFont = {
	meta: Font;
	url: string;
	hash: number;
};