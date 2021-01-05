import test from 'ava';
import { calculateWindChill } from './util';

test('calculateWindChill() should return correct wind chill', (t) => {
  t.is(calculateWindChill(38, 3), 36);
});

test('calculateWindChill() should return the default temp if windchill is below 3', (t) => {
  t.is(calculateWindChill(38, 2), 38);
});
