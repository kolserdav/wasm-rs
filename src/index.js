import { say, initSync } from '../app/pkg/wasm_app.js';

window.onload = () => {
  initSync();
  say('Test');
};
