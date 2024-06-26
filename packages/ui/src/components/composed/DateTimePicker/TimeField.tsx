'use client';

import React from 'react';
import { useLocale, useTimeField, type AriaTimeFieldProps, type TimeValue } from 'react-aria';
import { useTimeFieldState } from 'react-stately';
import { cn } from '@/utils';

import { DateSegment } from './DateSegment';
import { formatTimeFieldSegments } from './format-time-field-segments';

export const TimeField: React.FC<TProps> = (props) => {
	const { className, ...timeFieldProps } = props;
	const ref = React.useRef<HTMLDivElement | null>(null);
	const { locale } = useLocale();
	const state = useTimeFieldState({ ...timeFieldProps, locale });
	const { fieldProps } = useTimeField(timeFieldProps, state, ref);
	const segments = React.useMemo(() => formatTimeFieldSegments(state.segments), [state.segments]);

	return (
		<div
			{...fieldProps}
			className={cn(
				'border-input placeholder:text-muted-foreground focus-visible:ring-ring flex h-9 w-full rounded-md border bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50',
				timeFieldProps.isDisabled ? 'cursor-not-allowed opacity-50' : '',
				className
			)}
			ref={ref}
		>
			{segments.map((segment, i) => (
				<DateSegment key={i} segment={segment} state={state} />
			))}
		</div>
	);
};

interface TProps extends AriaTimeFieldProps<TimeValue> {
	className?: string;
}
