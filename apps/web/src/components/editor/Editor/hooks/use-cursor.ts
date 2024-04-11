import React from 'react';
import type { COMP } from '@dyn/dtif-comp';
import type { Composition } from '@dyn/svg-comp';

import { useOutputEvent } from './use-output-event';

export function useCursor(composition?: Composition): COMP.Cursor {
	const [cursor, setCursor] = React.useState<COMP.Cursor>({
		type: 'Default'
	});
	useOutputEvent(
		composition,
		'CursorChange',
		(event) => {
			setCursor(event.cursor);
		},
		[]
	);
	return cursor;
}
