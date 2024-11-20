/* Monaco Editor Helper */

import * as monaco from 'monaco-editor';

export default monaco;

import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';

const WORKERS: Record<string, Worker> = {};

self.MonacoEnvironment = {
	getWorker: function (id: string, label: string) {
		if (!WORKERS[id]) {
			switch (label) {
				default:
					WORKERS[id] = new editorWorker();
			}
		}

		return WORKERS[id];
	}
};
