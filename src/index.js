import init, { say, say_hello } from '../app/pkg/wasm_app.js';

window.onload = async () => {
  await init();
  const res = say('Test');
  console.log(res);
  const r = say_hello();

  console.log(r);
};
