import './plugin-handler';

// Init UI
figma.showUI(__html__);
if (process.env.PREVIEW_MODE) {
	figma.ui.resize(50, 50);
} else {
	figma.ui.resize(400, 600);
}
