import { $, $$ } from '@sciter';
import * as sys from '@sys';
import MOTION from 'this://app/js/motion.js';
import movableView from 'this://app/js/movable_view.js';

globalThis.CHOSEN_CLOSED_MOUTH_ANIMATION = 'vibing';
globalThis.CHOSEN_OPEN_MOUTH_ANIMATION = 'bouncy';

globalThis.MOUTH_IS_OPEN = false;
globalThis.BLINKING = false;

function getCurrentAvatarFrame() {
  const url = document.style
    .variables()
    [
      `${globalThis.MOUTH_IS_OPEN ? 'open' : 'closed'}${
        globalThis.BLINKING ? '-blink' : ''
      }`
    ]?.match(/(?<=\().*(?=\))/)?.[0];

  return url || $('#avatar').src;
}

adjust_window();
monitor_delay();
animate();
animateButton(
  $('#closed-mouth-motion'),
  globalThis.CHOSEN_CLOSED_MOUTH_ANIMATION
);
animateButton($('#open-mouth-motion'), globalThis.CHOSEN_OPEN_MOUTH_ANIMATION);
animateMouthButton();
blink();

function adjust_window() {
  const [width, height] = Window.this.screenBox('frame', 'dimension');
  const w = width * 0.666;
  const h = height * 0.666;
  Window.this.move(width / 2 - w / 2, height / 2 - h / 2, w, h, true);
}

update_meter();

function update_meter() {
  setInterval(() => {
    let volume = Window.this.xcall('get_volume');
    volume *= 100;
    $('#meter-microphone').style.variable('volume', `${volume}%`);

    const _delayVolume = $('#meter-delay').style.variables().volume;
    const _microphoneVolume = $('#meter-microphone').style.variables().volume;

    const delayVolume = +_delayVolume.replace('%', '');
    const delaySlider = $('#meter-delay').value;

    const microphoneVolume = +_microphoneVolume.replace('%', '');
    const microphoneSlider = $('#meter-microphone').value;

    if (microphoneVolume > microphoneSlider) {
      $('#meter-delay').style.variable('volume', '100%');
    } else {
      $('#meter-delay').style.variable(
        'volume',
        `${Math.max(0, delayVolume - 1)}%`
      );
    }

    if (delayVolume > delaySlider) {
      if (globalThis.MOUTH_IS_OPEN === false) {
        globalThis.MOUTH_IS_OPEN = true;
        //console.log('monitor delay => delayVolume > delaySlider');
        $('#avatar').src = getCurrentAvatarFrame();

        animate();
      }
    } else {
      if (globalThis.MOUTH_IS_OPEN === true) {
        globalThis.MOUTH_IS_OPEN = false;
        //console.log('monitor delay => else');
        $('#avatar').src = getCurrentAvatarFrame();
        animate();
      }
    }
  });
}

async function monitor_delay() {
  setInterval(() => {});
}

function animate() {
  let avatarCurr = $('#avatar').src;

  const animation = globalThis.MOUTH_IS_OPEN
    ? globalThis.CHOSEN_OPEN_MOUTH_ANIMATION
    : globalThis.CHOSEN_CLOSED_MOUTH_ANIMATION;

  globalThis.ANIMATION = animation;

  clearInterval(globalThis.ANIMATION_INTERVAL);

  globalThis.ANIMATION_INTERVAL = setInterval(() => {
    //console.log(globalThis.ANIMATION_INTERVAL);
    const t = Date.now() / 1000;
    const { x, y } = MOTION[animation](t);
    const left = -50 - x * 5;
    const top = -50 - y * 5;
    $('#avatar').style.transform = `translate(${left}%, ${top}%)`;

    avatarCurr = $('#avatar').src;

    //console.log(`${Date.now() / 1000}: ${globalThis.MOUTH_IS_OPEN}`);

    if (globalThis.MOUTH_IS_OPEN === false) {
      //const avatarNew = avatarCurr.replace('open', 'closed');

      const avatarNew = getCurrentAvatarFrame();

      if (avatarNew !== avatarCurr) {
        $('#avatar').src = avatarNew;
        avatarCurr = avatarNew;
      }
    } else {
      //const avatarNew = avatarCurr.replace('closed', 'open');

      const avatarNew = getCurrentAvatarFrame();

      if (avatarNew !== avatarCurr) {
        $('#avatar').src = avatarNew;
        avatarCurr = avatarNew;
      }
    }
  });
}

function animateButton(button, animation = 'motionless') {
  clearInterval(button.BUTTON_INTERVAL);

  button.BUTTON_INTERVAL = setInterval(() => {
    const t = Date.now() / 1000;
    const { x, y } = MOTION[animation](t);
    const left = 50 + x * 500;
    const top = 50 - y * 500;

    button.style.variable('x', `${left}%`);
    button.style.variable('y', `-${top + 500}%`);
  });
}

function blink() {
  let avatarCurr = $('#avatar').src;
  setInterval(() => {
    avatarCurr = $('#avatar').src;
    const n = Date.now() % 3200;
    if (n > 3000) {
      globalThis.BLINKING = true;
      /*const avatarNew = avatarCurr
        .replace('-blink', '')
        .replace('.png', '-blink.png');*/
      const avatarNew = getCurrentAvatarFrame();
      if (avatarCurr !== avatarNew) {
        $('#avatar').src = avatarNew;
        avatarCurr = avatarNew;
      }
    } else {
      globalThis.BLINKING = false;
      //const avatarNew = avatarCurr.replace('-blink', '');
      const avatarNew = getCurrentAvatarFrame();
      if (avatarCurr !== avatarNew) {
        $('#avatar').src = avatarNew;
        avatarCurr = avatarNew;
      }
    }
  });
}

async function _cycleAnimations() {
  const animations = Object.keys(MOTION);
  let i = 0;
  while (true) {
    const key = animations[i % animations.length];
    animate(key);
    Window.this.caption = key;
    i++;
    await new Promise((r) => setTimeout(r, 2000));
  }
}

document.on(
  '~mousedown',
  '#closed-mouth-motion, #open-mouth-motion',
  motionButtonEvent
);

document.on(
  '~doubleclick',
  '#closed-mouth-motion, #open-mouth-motion',
  motionButtonEvent
);

function motionButtonEvent(evt) {
  const { target: button } = evt;
  button.attributes.counter = button.attributes.counter || 0;

  if (evt.button === 1) {
    button.attributes.counter++;
  } else if (evt.button === 2) {
    button.attributes.counter--;
  }

  const color = [
    'white',
    '#9BCCD4',
    '#8087D6',
    '#AB65CF',
    '#E7FD5B',
    '#EC9F45',
    '#E24555',
  ][mod(button.attributes.counter, 7)];
  button.style.variable('color', color);

  const animation = [
    'motionless',
    'vibing',
    'shaking',
    'shakingMore',
    'bouncy',
    'excited',
    'nervous',
  ][mod(button.attributes.counter, 7)];

  animateButton(button, animation);

  if (button.id === 'closed-mouth-motion') {
    globalThis.CHOSEN_CLOSED_MOUTH_ANIMATION = animation;
    animate();
  } else if (button.id === 'open-mouth-motion') {
    globalThis.CHOSEN_OPEN_MOUTH_ANIMATION = animation;
    animate();
  }
}

function animateMouthButton() {
  setInterval(() => {
    const n = Date.now() % 1200;
    $('#mouth-transition').style.variable(
      'foreground',
      `url('this://app/png/controls/buttons/top/motion/${
        n > 600 ? 'open' : 'closed'
      }.png')`
    );
  });
}

function mod(n, m) {
  return ((n % m) + m) % m;
}

globalThis.CURRENT_BUTTON = null;

document.on(
  'click',
  '.mouth-image.border-default:not(:first-of-type)',
  (evt, el) => {
    globalThis.CURRENT_BUTTON = el;
    el.popup($('menu'));
  }
);

function changeImage(evt, el) {
  //setTimeout to avoid context menu popping up
  setTimeout(() => {
    if (el.matches('.mouth-image:first-of-type, .mouth-image.border-add')) {
      globalThis.CURRENT_BUTTON = el;
    }

    const filename = loadImage();
    if (!filename) return;
    const which = globalThis.CURRENT_BUTTON.id
      .match(/closed|open|blink/g)
      .join('-');
    document.style.variable(which, `url('${filename}')`);

    globalThis.CURRENT_BUTTON.classList.add('border-default');
    globalThis.CURRENT_BUTTON.classList.remove('border-add');
  });
}

document.on(
  'click',
  '.mouth-image:first-of-type, .mouth-image.border-add',
  changeImage
);

document.on('click', '#change-image', changeImage);

document.on('click', '#remove-image', (evt, el) => {
  globalThis.CURRENT_BUTTON.classList.remove('border-default');
  globalThis.CURRENT_BUTTON.classList.add('border-add');

  const which = globalThis.CURRENT_BUTTON.id
    .match(/closed|open|blink/g)
    .join('-');
  document.style.variable(which, null);
});

function loadImage() {
  const filename = Window.this.selectFile({
    mode: 'open',
    filter:
      'image files (*.bmp,*.dib,*.gif,*.png,*.apng,*.jpg,*.jpeg,*.jiff)|*.bmp;*.dib;*.gif;*.png;*.apng;*.jpg;*.jpeg;*.jiff',
    caption: 'select image for closed mouth...',
  });
  return filename;
}

movableView('body');

Window.this.on('activate', (evt) => {
  if (evt.reason === 0) {
    document.classList.add('transparent');
  } else {
    document.classList.remove('transparent');
  }
});

setInterval(() => (Window.this.isTopmost = true));

$('#about').on('click', () => {
  Window.this.modal({ url: 'this://app/html/about.html' });
});

$('#close').on('click', () => Window.this.close());