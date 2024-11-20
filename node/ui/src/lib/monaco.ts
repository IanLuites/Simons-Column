/* Monaco Editor Helper */

import * as monaco from 'monaco-editor';

export default monaco;

import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';

self.MonacoEnvironment = {
	getWorker: function (_: string, label: string) {
		switch (label) {
			default:
				return new editorWorker();
		}
	}
};
