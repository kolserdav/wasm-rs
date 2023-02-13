import init, { say, say_hello } from '../app/pkg/wasm_app.js';
import type { SayHello } from '../app/bindings/SayHello';
import "./styles/index.scss";

window.onload = async () => {
  await init(undefined);
  const res = say('Test');
  console.log(res);
  const {first, second} = say_hello() as SayHello;

  console.log({first, second});
};
