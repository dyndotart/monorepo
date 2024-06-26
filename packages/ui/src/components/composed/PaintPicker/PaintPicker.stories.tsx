import type { Meta, StoryObj } from '@storybook/react';
import React from 'react';

import { PaintPicker } from './index';

const meta: Meta<typeof PaintPicker> = {
	title: 'ui/PaintPicker',
	component: PaintPicker,
	tags: ['autodocs'],
	argTypes: {},
	render: (args) => {
		const [paint, setPaint] = React.useState(args.paint);
		return (
			<div className="w-48">
				<PaintPicker {...args} onPaintUpdate={setPaint} paint={paint} />
			</div>
		);
	},
	parameters: {
		layout: 'centered'
	},
	args: {
		// paint: { type: 'Solid', color: [240, 128, 128, 1] }
		paint: { type: 'Image', opacity: 1, scaleMode: { type: 'Fill' } }
	}
} satisfies Meta<typeof PaintPicker>;

export default meta;

type Story = StoryObj<typeof meta>;

/**
 * The default form of the paint picker.
 */
export const Default: Story = {};
