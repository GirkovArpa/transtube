import { pnoise1 } from 'this://app/js/perlin_noise.js';

export default {
  motionless,
  vibing,
  shaking,
  shakingMore,
  bouncy,
  excited,
  nervous,
};

function motionless(t) {
  return { x: 0, y: 0 };
}

function vibing(t) {
  return _shake(t, 0.7, 0.5);
}

function shaking(t) {
  return _shake(t, 0.3, 10);
}

function shakingMore(t) {
  return _shake(t, 1, 8);
}

function bouncy(t) {
  return _jumpy(t, 0, 0.5, 1);
}

function excited(t) {
  return _jumpy(t, 0.5, 0.5, 2);
}

function nervous(t) {
  return _jumpy(t, 1, 1, 4);
}

function clamp01(number) {
  return Math.max(0, Math.min(number, 1));
}

function _shake(t, amount, velocity) {
  let num = clamp01(pnoise1(t * velocity, 0));
  let num2 = clamp01(pnoise1(t * velocity, 50));
  num = num * 2 - 1;
  num2 = num2 * 2 - 1;
  return {
    x: amount * (num * Math.sqrt(1 - (num2 * num2) / 2)),
    y: amount * (num2 * Math.sqrt(1 - (num * num) / 2)),
  };
}

function _jumpy(t, amountX, amountY, velocity) {
  t *= velocity;
  const num = t % 1;
  const num2 = -8 * num * (num - 1) - 1;
  let num3 = t % 2;
  if (num3 > 1) {
    num3 = 2 - num3;
  }
  return { x: (num3 * 2 - 1) * amountX, y: num2 * amountY };
}
