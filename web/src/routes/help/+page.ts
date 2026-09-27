import type { PageLoad } from './$types';

export const load: PageLoad = () => {
	return {
		rippers: [
            {
                "name": "Exact Audio Copy",
                "platforms": ["Windows"],
                "link": "https://www.exactaudiocopy.de/",
                "experimental": false
            },
            {
                "name": "X Lossless Decoder",
                "platforms": ["Mac"],
                "link": "https://tmkk.undo.jp/xld/index_e.html",
                "experimental": false
            },
            {
                "name": "whipper",
                "platforms": ["Linux"],
                "link": "https://github.com/whipper-team/whipper",
                "experimental": false
            },
            {
                "name": "CUERipper",
                "platforms": ["Windows"],
                "link": "https://github.com/gchudov/cuetools.net",
                "experimental": true
            },
            {
                "name": "cyanrip",
                "platforms": ["Linux", "Windows"],
                "link": "https://github.com/cyanreg/cyanrip",
                "experimental": true
            },
            {
                "name": "dBpoweramp",
                "platforms": ["Windows", "Mac"],
                "link": "https://www.dbpoweramp.com/",
                "experimental": true
            },
            {
                "name": "EZ CD Audio Converter",
                "platforms": ["Windows"],
                "link": "https://www.poikosoft.com/",
                "experimental": true
            },
            {
                "name": "fre:ac",
                "platforms": ["Windows", "Mac", "Linux"],
                "link": "https://www.freac.org/",
                "experimental": true
            },
            {
                "name": "morituri",
                "platforms": ["Linux"],
                "link": "https://github.com/thomasvs/morituri",
                "experimental": true
            },
            {
                "name": "Rip",
                "platforms": ["Mac"],
                "link": "https://github.com/sbooth/Rip",
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
