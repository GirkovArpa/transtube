import { $, $$ } from '@sciter';
import { launch } from '@env';

$('#sciter').on('click', () => {
  launch('https://sciter.com/?ref=transtube');
});

$('#terra-informatica').on('click', () => {
  launch('https://terrainformatica.com/?ref=transtube');
});

$('#veadotube-mini').on('click', () => {
  launch('https://veado.tube/?ref=transtube');
});

$('#girkov-arpa').on('click', () => {
  launch('https://girkovarpa.itch.io/?ref=transtube');
});

$('button').on('click', () => Window.this.close());
