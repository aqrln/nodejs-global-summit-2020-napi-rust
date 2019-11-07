const benchmark = require('benchmark');
const addon = require('./build/Release/example2.node');

function createArray(n) {
  const array = new Float64Array(n);
  for (let i = 0; i < n; i++) {
    array[i] = Math.random();
  }
  return array;
}

function sumOfSquaresImperative(array) {
  let result = 0.0;
  for (const x of array) {
    result += x * x;
  }
  return result;
}

function sumOfSquaresFunctional(array) {
  return array.map(x => x * x).reduce((x, y) => x + y);
}

const array = createArray(1000000);
let blackhole = 0.0;

new benchmark.Suite()
  .add('JS (imperative)', () => {
    blackhole += sumOfSquaresImperative(array);
  })
  .add('JS (functional)', () => {
    blackhole += sumOfSquaresFunctional(array);
  })
  .add('Rust (parallel)', () => {
    blackhole += addon.sumOfSquaresPar(array);
  })
  .add('Rust (sequential)', () => {
    blackhole += addon.sumOfSquaresSeq(array);
  })
  .on('cycle', event => {
    console.log(String(event.target));
  })
  .on('complete', function () {
    console.log('Fastest is ' + this.filter('fastest').map('name'));
  })
  .run();

console.log(blackhole); // to prevent smart VMs from throwing away the actual benchmark code
