import React from 'react';

import type { TIconProps } from './types';

export const ScribbleRepeatIcon = React.forwardRef<SVGSVGElement, TIconProps>((props, ref) => {
	return (
		<svg ref={ref} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" {...props}>
			<path
				xmlns="http://www.w3.org/2000/svg"
				d="M3.29091 16.878C2.81229 17.2013 2.55217 17.4071 2.27124 17.5737C1.95909 17.7598 1.62613 17.7794 1.30358 17.5737C0.929006 17.3385 0.658478 17.0054 0.762525 16.5938C1.1475 15.0653 1.74057 13.5956 2.79146 12.3512C3.23886 11.8123 3.93599 11.6359 4.67473 11.8907C4.87243 11.9593 5.09093 11.9887 5.25741 12.0964C6.87017 13.1057 8.47252 14.1247 10.0645 15.1633C10.2726 15.3005 10.387 15.5748 10.5327 15.7708C10.335 16.535 9.78355 16.7702 9.11764 16.5938C8.46213 16.4273 7.83784 16.1235 7.21354 15.8296C6.67249 15.5748 6.15224 15.2711 5.47592 14.9085C5.22621 16.0549 5.56958 16.9564 5.87132 17.8578C6.20428 18.8082 6.81817 19.5921 7.66097 20.2094C9.71074 21.6987 13.7374 22.3846 16.5363 20.1408C19.252 17.9656 20.8647 15.2417 21.4058 11.9691C21.7699 9.77424 21.1872 7.67741 20.0739 5.74715C19.0959 4.06185 17.4103 3.27799 15.4958 3.02324C12.4055 2.62151 9.41932 3.13101 6.5892 4.36559C5.36143 4.9045 4.36256 5.68836 4.34176 7.1581C4.34176 7.32467 4.0088 7.59902 3.80071 7.61862C3.5718 7.63821 3.25965 7.49124 3.09317 7.32467C2.53131 6.70738 2.55211 5.95291 2.89547 5.30623C3.25964 4.62035 3.74867 3.94427 4.35215 3.44455C5.01806 2.89585 5.82963 2.47453 6.65162 2.14139C8.96149 1.20076 11.417 0.740234 13.9246 0.740234C15.0796 0.740234 16.2761 0.936197 17.3894 1.23994C19.9282 1.93562 21.6971 3.48375 22.6855 5.83534C24.8186 10.901 23.5804 15.4376 20.1884 19.5823C19.3561 20.6013 18.2739 21.4734 17.171 22.2474C12.6449 25.4515 4.89329 23.2763 3.55104 17.8382C3.51982 17.5639 3.43658 17.3287 3.29091 16.878Z"
				fill="currentColor"
			/>
		</svg>
	);
});
ScribbleRepeatIcon.displayName = 'Scribble Repeat Icon';
