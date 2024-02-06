import type { PageLoad } from './$types';

export const load: PageLoad = () => {
	return {
		rippers: [
            {
                "name": "Exact Audio Copy",
                "platform": "Windows",
                "link": "https://www.exactaudiocopy.de/",
                "experimental": false
            },
            {
                "name": "X Lossless Decoder",
                "platform": "Mac",
                "link": "https://tmkk.undo.jp/xld/index_e.html",
                "experimental": false
            },
            {
                "name": "whipper",
                "platform": "Linux",
                "link": "https://github.com/whipper-team/whipper",
                "experimental": false
            },
            {
                "name": "CUERipper",
                "platform": "Windows",
                "link": "https://github.com/gchudov/cuetools.net",
                "experimental": true
            }
        ],
        evaluators: [
            {
                "name": "Orpheus",
                "link": "https://github.com/OPSnet/Logchecker",
                "experimental": false
            }
        ]        
	};
};