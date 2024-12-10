import {Pane} from 'https://cdn.jsdelivr.net/npm/tweakpane@4.0.5/dist/tweakpane.min.js';
const pane = new Pane();

const PARAMS = {
  factor: 123,
  title: 'hello',
  color: '#ff0055',
  size: 10
};

const b = pane.addBinding(
  PARAMS, 'size',
  {min: 8, max: 100, step: 1}
);

b.on('change', function(ev) {
  console.log(`change: ${ev.value}`);
});

pane.addBinding(PARAMS, 'factor');
pane.addBinding(PARAMS, 'title');
pane.addBinding(PARAMS, 'color');
pane.addBinding(PARAMS, 'size');